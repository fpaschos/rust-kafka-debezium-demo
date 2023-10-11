use crate::proto_convert_enum::ProtoConvertEnum;
use crate::proto_convert_struct::ProtoConvertStruct;
use darling::FromDeriveInput;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

pub fn implement_proto_convert(input: TokenStream) -> TokenStream {
    let input = ProtoConvert::from_derive_input(&syn::parse(input.into()).unwrap())
        .unwrap_or_else(|e| panic!("ProtoConvert: {}", e));
    quote! { #input }
}

enum ProtoConvert {
    Struct(ProtoConvertStruct),
    Enum(ProtoConvertEnum),
}

impl ProtoConvert {
    fn name(&self) -> &Ident {
        match self {
            Self::Struct(inner) => &inner.name,
            Self::Enum(inner) => &inner.name,
        }
    }
    fn implement_proto_convert(&self) -> impl ToTokens {
        match self {
            Self::Struct(data) => quote! { #data },
            Self::Enum(data) => quote! { #data },
        }
    }
}

impl ToTokens for ProtoConvert {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mod_name = Ident::new(
            &format!("proto_convert_impl_{}", self.name()),
            Span::call_site(),
        );
        let proto_convert = self.implement_proto_convert();

        let expanded = quote! {
            mod #mod_name {
                use super::*;

                use protobuf::Message as _ProtobufMessage;

                #proto_convert
            }
        };

        tokens.extend(expanded)
    }
}

impl darling::FromDeriveInput for ProtoConvert {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        match &input.data {
            Data::Struct(data) => Ok(ProtoConvert::Struct(ProtoConvertStruct::from_derive_input(
                input.ident.clone(),
                input.attrs.as_ref(),
                data,
            )?)),
            Data::Enum(data) => Ok(ProtoConvert::Enum(ProtoConvertEnum::from_derive_input(
                input.ident.clone(),
                input.attrs.as_ref(),
                data,
            )?)),
            _ => Err(darling::Error::unsupported_shape(
                "Macro supports only `struct` and `enum` items",
            )),
        }
    }
}
