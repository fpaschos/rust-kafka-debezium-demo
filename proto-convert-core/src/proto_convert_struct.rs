use crate::attributes::{ProtoConvertFieldAttrs, ProtoConvertStructAttrs};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, DataStruct};

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

    /// Implementation of `proto_convert` for rust `struct` items
    fn impl_proto_convert(&self) -> TokenStream {
        let name = &self.name;
        let proto_struct = self.attrs.source.as_ref();

        let to_proto_impl = {
            let fields = self
                .fields
                .iter()
                .map(|(ident, attrs)| attrs.impl_struct_field_getter(ident));

            quote! {
                let mut msg = Self::ProtoStruct::default();
                #(#fields)*
                msg
            }
        };

        let from_proto_impl = {
            let fields = self
                .fields
                .iter()
                .map(|(ident, attrs)| attrs.impl_struct_field_setter(ident));

            quote! {
                let inner = Self {
                    #(#fields)*
                };
                Ok(inner)
            }
        };

        quote! {
            impl ProtoConvert for #name {
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

impl ToTokens for ProtoConvertStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expanded = self.impl_proto_convert();
        tokens.extend(expanded);
    }
}
