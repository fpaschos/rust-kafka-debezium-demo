use crate::proto_enum::Enum;
use crate::proto_struct::Struct;
use darling::FromDeriveInput;
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput};

pub fn implement_proto_convert(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input.into()).unwrap();
    let input =
        ProtoConvert::from_derive_input(&input).unwrap_or_else(|e| panic!("ProtoConvert: {}", e));
    quote! { #input }
}

enum ProtoConvert {
    Struct(Struct),
    Enum(Enum),
}

impl ProtoConvert {
    fn name(&self) -> &Ident {
        match self {
            Self::Struct(inner) => &inner.name,
            Self::Enum(inner) => &inner.name,
        }
    }
    fn implement_proto_convert(&self) -> TokenStream {
        match self {
            Self::Struct(data) => data.implement_proto_convert(),
            Self::Enum(data) => data.implement_proto_convert(),
        }
    }
}

impl ToTokens for ProtoConvert {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_name = format_ident!(
            "proto_convert_impl_{}",
            self.name().to_string().to_snake_case()
        );

        let proto_convert = self.implement_proto_convert();

        let expanded = quote! {
            mod #mod_name {
                use super::*;

                use protobuf::Message as _ProtobufMessage; // TODO do we need this?


                #proto_convert
            }
        };

        tokens.extend(expanded)
    }
}

impl darling::FromDeriveInput for ProtoConvert {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        match &input.data {
            Data::Struct(data) => {
                let s = Struct::try_from_data(&input.ident, data, &input.attrs)?;
                Ok(ProtoConvert::Struct(s))
            }
            Data::Enum(data) => Ok(ProtoConvert::Enum(Enum::from_derive_input(
                input.ident.clone(),
                data,
                input.attrs.as_ref(),
            )?)),
            _ => Err(darling::Error::unsupported_shape(
                "Macro supports only `struct` and `enum` items",
            )),
        }
    }
}
