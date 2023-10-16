use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Path};

use crate::types::{ScalarTy, Ty};
use crate::{find_proto_convert_meta, get_proto_field_name};

pub(crate) struct StructField {
    pub name: Ident,
    pub ty: Ty,
    pub attrs: Option<FieldAttrs>,
}

#[cfg(test)]
impl StructField {
    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        self.ty.is_optional()
    }
}

impl StructField {
    pub(crate) fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        let name = field.ident.as_ref().ok_or_else(|| {
            darling::Error::unsupported_shape("Macro supports only structs with named fields")
        })?;

        let meta = find_proto_convert_meta(&field.attrs);
        let attrs = if let Some(meta) = meta {
            Some(FieldAttrs::from_meta(meta)?)
        } else {
            None
        };

        let ty = Ty::try_from_field(field)?;

        Ok(Self {
            name: name.clone(),
            ty,
            attrs,
        })
    }

    // TODO use struct attrs for rename_all
    pub(crate) fn implement_getter(&self, _struct_attrs: &StructAttrs) -> TokenStream {
        // Fast handle skip attribute
        if let Some(FieldAttrs { skip: true, .. }) = &self.attrs {
            return quote! {};
        }

        //Check field rename
        let proto_field_setter = if let Some(FieldAttrs {
            rename: Some(new_name),
            ..
        }) = &self.attrs
        {
            let field_name = get_proto_field_name(new_name.as_str(), Some('_'));
            format_ident!("set_{}", field_name)
        } else {
            format_ident!("set_{}", &self.name)
        };

        let struct_field = &self.name;

        let to_proto_method = if self.ty.is_scalar() {
            quote! { ProtoConvertScalar::to_scalar }
        } else {
            quote! { ProtoConvert::to_proto }
        };

        if self.ty.is_optional() {
            // Optional field setter
            quote! {
                if let Some(value) = &self.#struct_field {
                    proto.#proto_field_setter(#to_proto_method(value).into());
                }
            }
        } else {
            // Non optional field just a setter
            quote! {
                proto.#proto_field_setter(#to_proto_method(&self.#struct_field).into());
            }
        }
    }

    // TODO use struct attrs for rename_all
    pub(crate) fn implement_setter(&self, _struct_attrs: &StructAttrs) -> TokenStream {
        let struct_field = &self.name;

        // Fast fail skip attribute
        if let Some(FieldAttrs { skip: true, .. }) = &self.attrs {
            // Default struct setter for the skipped fields.
            return quote! { #struct_field: Default::default(), };
        }

        //Check field rename
        let proto_field = if let Some(FieldAttrs {
            rename: Some(new_name),
            ..
        }) = &self.attrs
        {
            let field_name = get_proto_field_name(new_name.as_str(), None);
            format_ident!("{}", field_name)
        } else {
            struct_field.clone() // Here proto and struct field are the same
        };

        let from_proto_method = if self.ty.is_scalar() {
            quote! { ProtoConvertScalar::from_scalar }
        } else {
            quote! { ProtoConvert::from_proto }
        };

        let proto_field_getter = format_ident!("{}", proto_field);

        if self.ty.is_optional() {
            // Determine the appropriate has_value method
            let has_value_check = match self.ty {
                Ty::Scalar {
                    ty: ScalarTy::Enumeration,
                    ..
                } => quote! {
                    ProtoScalar::has_value(&value.value())
                },
                Ty::Scalar { .. } => quote! {
                    ProtoScalar::has_value(&value)
                },
                Ty::Other { .. } => {
                    let has_field = format_ident!("has_{}", proto_field);
                    quote! { proto.#has_field() }
                }
            };

            // In case of optional check value is empty via `ProtoPrimitiveValue::has_value(..)`
            quote! {
                #struct_field: {
                    let value = proto.#proto_field_getter().to_owned();
                    if #has_value_check {
                        Some(#from_proto_method(value)?)
                    } else {
                        None
                    }
                },
            }
        } else {
            // Non optional field just a setter
            quote! {
                #struct_field: #from_proto_method(proto.#proto_field_getter().to_owned())?,
            }
        }
    }
}

pub(crate) struct Struct {
    pub name: Ident,
    pub attrs: StructAttrs,
    pub fields: Vec<StructField>,
}

impl Struct {
    pub(crate) fn try_from_data(
        name: &Ident,
        data: &DataStruct,
        attrs: &[Attribute],
    ) -> darling::Result<Self> {
        let meta = find_proto_convert_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing required proto attribute `proto_convert`")
        })?;

        let attrs = StructAttrs::from_meta(meta)?;

        let fields = data
            .fields
            .iter()
            .map(StructField::try_from_field)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: name.clone(),
            fields,
            attrs,
        })
    }

    pub(crate) fn implement_proto_convert(&self) -> TokenStream {
        let struct_name = format_ident!("{}", &self.name);
        let proto_struct = &self.attrs.source;
        let to_proto_impl = {
            let fields = self.fields.iter().map(|f| f.implement_getter(&self.attrs));

            quote! {
                let mut proto = #proto_struct::default();
                #(#fields)*
                proto
            }
        };

        let from_proto_impl = {
            let fields = self.fields.iter().map(|f| f.implement_setter(&self.attrs));

            quote! {
                let inner = Self {
                    #(#fields)*
                };
                Ok(inner)
            }
        };

        quote! {
            impl ProtoConvert for #struct_name {
                type ProtoStruct = #proto_struct;
                fn to_proto(&self) -> Self::ProtoStruct {
                    #to_proto_impl
                }

                fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                    #from_proto_impl
                }
            }
        }
    }
}

/// Meta attributes for `struct` items
#[derive(Debug, darling::FromMeta)]
pub(crate) struct StructAttrs {
    pub source: Path,
    /// Optional renaming of the struct fields before mapping to the proto entity.
    pub rename_all: Option<String>,
}

/// Meta attributes for `struct field` items
#[derive(Debug, darling::FromMeta, Default)]
#[darling(default)]
pub(crate) struct FieldAttrs {
    /// Optional skipping struct field from proto serialization
    pub skip: bool,
    /// Optional mark the field as an enumeration mapping (used only for optional getter/setter mapping)
    pub enumeration: bool,
    /// Optional renaming of a single struct field before mapping to the proto entity.
    pub rename: Option<String>,
}
