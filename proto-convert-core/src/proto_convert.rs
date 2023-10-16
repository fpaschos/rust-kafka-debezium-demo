use crate::proto_convert_enum::ProtoConvertEnum;
use crate::proto_struct::Struct;
use darling::FromDeriveInput;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

pub fn implement_proto_convert(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input.into()).unwrap();
    let input =
        ProtoConvert::from_derive_input(&input).unwrap_or_else(|e| panic!("ProtoConvert: {}", e));
    quote! { #input }
}

enum ProtoConvert {
    Struct(Struct),
    Enum(ProtoConvertEnum),
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
