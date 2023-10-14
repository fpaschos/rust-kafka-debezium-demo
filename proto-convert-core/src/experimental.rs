use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataStruct, DeriveInput, GenericArgument, Path, PathArguments, Type};

use crate::find_proto_convert_meta;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum PrimitiveTy {
    F32,
    F64,
    U32,
    I32,
    U64,
    I64,
    Bool,
    String,
    VecBytes, // TODO how to handle this?
    // Special case for enumerations fall back to u32
    Enumeration,
}

pub(crate) fn maybe_known_proto_primitive(ident: &Ident) -> Option<PrimitiveTy> {
    match ident {
        _ if ident == "u32" => Some(PrimitiveTy::U32),
        _ if ident == "i32" => Some(PrimitiveTy::I32),
        _ if ident == "f32" => Some(PrimitiveTy::F32),
        _ if ident == "f64" => Some(PrimitiveTy::F64),
        _ if ident == "u64" => Some(PrimitiveTy::U64),
        _ if ident == "i64" => Some(PrimitiveTy::I64),
        _ if ident == "bool" => Some(PrimitiveTy::Bool),
        _ if ident == "String" => Some(PrimitiveTy::String),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Ty {
    Primitive { ty: PrimitiveTy, optional: bool },
    Other { ty: Path, optional: bool },
}

impl Ty {
    pub(crate) fn primitive(ty: PrimitiveTy, optional: bool) -> Self {
        Self::Primitive { ty, optional }
    }

    pub(crate) fn other(ty: Path, optional: bool) -> Self {
        Self::Other { ty, optional }
    }

    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        match self {
            Ty::Primitive { optional, .. } => *optional,
            Ty::Other { optional, .. } => *optional,
        }
    }

    #[inline]
    pub(crate) fn is_primitive(&self) -> bool {
        matches!(self, Ty::Primitive { .. })
    }

    // TODO handle enumeration case via attrs
    fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        match &field.ty {
            Type::Path(syn::TypePath { path, .. }) => {
                let last_segment = path.segments.last().unwrap(); // TODO what if there is no last segment
                return match &last_segment.arguments {
                    PathArguments::None => {
                        // Check only for primitive types or any other type
                        if let Some(ty) = maybe_known_proto_primitive(&last_segment.ident) {
                            Ok(Ty::primitive(ty, false))
                        } else {
                            Ok(Ty::other(path.clone(), false))
                        }
                    }
                    PathArguments::AngleBracketed(args) => {
                        if args.args.iter().count() > 1 {
                            // Types with more than one generic argument are classified as other
                            Ok(Ty::other(path.clone(), false))
                        } else if last_segment.ident == "Option" {
                            // Type is option check the inner type
                            let gen = args.args.iter().next().unwrap();

                            if let GenericArgument::Type(Type::Path(t)) = gen {
                                let last_segment = t.path.segments.last().unwrap(); // TODO what if there is no last segment
                                                                                    // Check only for primitive types or any other type
                                if let Some(ty) = maybe_known_proto_primitive(&last_segment.ident) {
                                    Ok(Ty::primitive(ty, true))
                                } else {
                                    Ok(Ty::other(path.clone(), true))
                                }
                            } else {
                                Err(
                                    darling::Error::unexpected_type(
                                        "Macro only supports path argument types",
                                    ), // .with_span(&field.span()), TODO test this
                                )
                            }
                        } else {
                            // The type is not Option return it as other non optional
                            Ok(Ty::other(path.clone(), false))
                        }
                    }

                    PathArguments::Parenthesized(_) => {
                        Err(
                            darling::Error::unexpected_type(
                                "Macro does not support parenthesized type arguments",
                            ), // .with_span(&field.span()), TODO test this
                        )
                    }
                };
            }
            _ => Err(
                darling::Error::unexpected_type("Macro supports only primitive path types"), // .with_span(&field.span()), TODO test this
            ),
        }
    }
}

pub(crate) struct StructField {
    pub name: String, // TODO keep ident of field here for better error handling
    pub ty: Ty,
    pub attrs: Option<FieldAttrs>,
}

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
            name: name.to_string(),
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

        // TODO extract final proto field name from attrs
        let proto_field_name = &self.name;

        let field_getter = quote::format_ident!("{}", self.name);
        let proto_field_setter = quote::format_ident!("set_{}", proto_field_name);

        let to_proto_method = if self.ty.is_primitive() {
            quote! { ProtoConvertPrimitive::to_primitive }
        } else {
            quote! { ProtoConvert::to_proto }
        };

        if self.ty.is_optional() {
            // Optional field setter
            quote! {
                if let Some(value) = &self.#field_getter {
                    proto.#proto_field_setter(#to_proto_method(value).into());
                }
            }
        } else {
            // Non optional field just a setter
            quote! {
                proto.#proto_field_setter(#to_proto_method(&self.#field_getter).into());
            }
        }
    }

    // TODO use struct attrs for rename_all
    pub(crate) fn implement_setter(&self, _struct_attrs: &StructAttrs) -> TokenStream {
        let struct_field = format_ident!("{}", self.name);

        // TODO extract final proto field name from attrs
        let proto_field_getter = struct_field.clone(); // Here proto and struct field are the same

        // Fast fail skip attribute
        if let Some(FieldAttrs { skip: true, .. }) = &self.attrs {
            // Default struct setter for the skipped fields.
            return quote! { #struct_field: Default::default(), };
        }

        let from_proto_method = if self.ty.is_primitive() {
            quote! { ProtoConvertPrimitive::from_primitive }
        } else {
            quote! { ProtoConvert::from_proto }
        };

        if self.ty.is_optional() {
            // Determine the appropriate has value method
            let has_value_check = match self.ty {
                Ty::Primitive {
                    ty: PrimitiveTy::Enumeration,
                    ..
                } => quote! {
                    ProtoPrimitive::has_value(&value.value())
                },
                Ty::Primitive { .. } => quote! {
                    ProtoPrimitive::has_value(&value)
                },
                Ty::Other { .. } => {
                    let has_field = format_ident!("has_{}", struct_field);
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
    pub name: String, // TODO keep ident of struct name here for better error handling
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
            name: name.clone().to_string(),
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
            impl ProtoConvert<#proto_struct> for #struct_name {

                fn to_proto(&self) -> #proto_struct {
                    #to_proto_impl
                }

                fn from_proto(proto: #proto_struct) -> std::result::Result<Self, anyhow::Error> {
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

pub(crate) fn from_derive_input(input: &DeriveInput) -> darling::Result<Struct> {
    match &input.data {
        Data::Struct(data) => {
            let s = Struct::try_from_data(&input.ident, data, &input.attrs)?;
            Ok(s)
        }

        //     Ok(ProtoConvert::Struct(ProtoConvertStruct::from_derive_input(
        //     input.ident.clone(),
        //     input.attrs.as_ref(),
        //     data,
        // )?)),
        _ => Err(darling::Error::unsupported_shape(
            "Macro supports only `struct` and `enum` items",
        )),
    }
}
