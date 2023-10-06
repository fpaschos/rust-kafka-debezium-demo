use crate::find_proto_convert_meta;
use crate::proto_convert_enum::ProtoConvertEnum;
use crate::proto_convert_struct::ProtoConvertStruct;
use darling::{FromDeriveInput, FromMeta};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DeriveInput};

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

#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub(crate) struct ProtoConvertFieldAttrs {
    skip: bool,
    // with: Option<Path>,
}

impl ProtoConvertFieldAttrs {
    // TODO support skip
    pub(crate) fn impl_field_setter(&self, ident: &Ident) -> impl ToTokens {
        let proto_getter = Ident::new(&ident.to_string(), Span::call_site());

        let setter = if self.skip {
            // Default setter for the skipped fields.
            quote! { Default::default() }
        } else {
            // Usual setter.
            quote! { ProtoConvert::from_proto(proto.#proto_getter().to_owned())? }
        };

        // let setter = match (self.skip, &self.with) {
        //     // Usual setter.
        //     (false, None) => quote! { ProtobufConvert::from_pb(pb.#pb_getter().to_owned())? },
        //     // Setter with the overridden Protobuf conversion.
        //     (false, Some(with)) => quote! { #with::from_pb(pb.#pb_getter().to_owned())? },
        //     // Default setter for the skipped fields.
        //     (true, _) => quote! { Default::default() },
        // };

        quote! { #ident: #setter, }
    }

    pub(crate) fn impl_field_getter(&self, ident: &Ident) -> impl ToTokens {
        let proto_setter = Ident::new(&format!("set_{}", ident), Span::call_site());
        if self.skip {
            // Skipped getter does nothing.
            quote! {}
        } else {
            // Usual getter.
            quote! {
                msg.#proto_setter(ProtoConvert::to_proto(&self.#ident).into());
            }
        }

        // match (self.skip, &self.with) {
        //     // Usual getter.
        //     (false, None) => quote! {
        //         msg.#pb_setter(ProtobufConvert::to_pb(&self.#ident).into());
        //     },
        //     // Getter with the overridden Protobuf conversion.
        //     (false, Some(with)) => quote! {
        //         msg.#pb_setter(#with::to_pb(&self.#ident).into());
        //     },
        //     // Skipped getter does nothing.
        //     (true, _) => quote! {},
        // }
    }
}

impl TryFrom<&[Attribute]> for ProtoConvertFieldAttrs {
    type Error = darling::Error;

    fn try_from(attrs: &[Attribute]) -> Result<Self, Self::Error> {
        find_proto_convert_meta(attrs)
            .map(Self::from_meta)
            .unwrap_or_else(|| Ok(Self::default()))
    }
}
