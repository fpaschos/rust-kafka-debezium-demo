use crate::find_proto_convert_meta;
use crate::impl_proto_convert::ProtoConvertFieldAttrs;
use darling::FromMeta;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{Attribute, DataStruct, Path};

#[derive(Debug)]
pub(crate) struct ProtoConvertStruct {
    pub name: Ident,
    pub attrs: ProtoConvertStructAttrs,
    pub fields: Vec<(Ident, ProtoConvertFieldAttrs)>,
}

impl ProtoConvertStruct {
    pub(crate) fn from_derive_input(
        name: Ident,
        attrs: &[Attribute],
        data: &DataStruct,
    ) -> darling::Result<Self> {
        let fields = Self::get_fields_with_attrs(data)?;

        let attrs = ProtoConvertStructAttrs::try_from(attrs)?;

        if attrs.source.is_none() {
            return Err(darling::Error::unsupported_shape(
                "Missing `source` meta field",
            ));
        }
        Ok(Self {
            name,
            attrs,
            fields,
        })
    }

    fn get_fields_with_attrs(
        data: &DataStruct,
    ) -> darling::Result<Vec<(Ident, ProtoConvertFieldAttrs)>> {
        data.fields
            .iter()
            .map(|f| {
                let ident = f.ident.clone().ok_or_else(|| {
                    darling::Error::unsupported_shape("Struct fields must have identifiers.")
                })?;

                let attrs = ProtoConvertFieldAttrs::try_from(f.attrs.as_ref())?;
                Ok((ident, attrs))
            })
            .collect()
    }
}

impl ToTokens for ProtoConvertStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let proto_struct = self.attrs.source.as_ref();

        let from_proto_impl = {
            let fields = self
                .fields
                .iter()
                .map(|(ident, attrs)| attrs.impl_field_setter(ident));

            quote! {
                let inner = Self {
                    #(#fields)*
                };
                Ok(inner)
            }
        };

        let to_proto_impl = {
            let fields = self
                .fields
                .iter()
                .map(|(ident, attrs)| attrs.impl_field_getter(ident));

            quote! {
                let mut msg = Self::ProtoStruct::default();
                #(#fields)*
                msg
            }
        };

        let expanded = quote! {
            impl ProtoConvert for #name {
                type ProtoStruct = #proto_struct;

                fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                    #from_proto_impl
                }

                fn to_proto(&self) -> Self::ProtoStruct {
                    #to_proto_impl
                }
            }
        };
        tokens.extend(expanded);
    }
}

#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub(crate) struct ProtoConvertStructAttrs {
    source: Option<Path>,
}

impl TryFrom<&[Attribute]> for ProtoConvertStructAttrs {
    type Error = darling::Error;

    fn try_from(attrs: &[Attribute]) -> Result<Self, Self::Error> {
        let meta = find_proto_convert_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing meta attribute `proto_convert`")
        })?;
        Self::from_meta(meta)
    }
}
