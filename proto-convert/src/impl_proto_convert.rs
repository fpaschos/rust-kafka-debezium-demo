use crate::find_proto_convert_meta;
use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::ReturnType::Default;
use syn::{Attribute, Data, DataStruct, DeriveInput, Path};

pub(crate) fn implement_proto_convert(input: TokenStream) -> TokenStream {
    let input = ProtoConvert::from_derive_input(&syn::parse(input).unwrap())
        .unwrap_or_else(|e| panic!("ProtoConvert: {}", e));
    let tokens = quote! { #input };
    tokens.into()
}

enum ProtoConvert {
    Struct(ProtoConvertStruct),
}

impl ProtoConvert {
    fn name(&self) -> &Ident {
        match self {
            Self::Struct(inner) => &inner.name,
        }
    }
    fn implement_proto_convert(&self) -> impl ToTokens {
        match self {
            Self::Struct(data) => quote! { #data },
        }
    }
}

impl ToTokens for ProtoConvert {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
            // Data::Enum(data) => {},
            _ => Err(darling::Error::unsupported_shape(
                "Supports only  Struct items",
            )),
        }
    }
}

#[derive(Debug)]
struct ProtoConvertStruct {
    name: Ident,
    attrs: ProtoConvertStructAttrs,
    fields: Vec<(Ident, ProtoConvertFieldAttrs)>,
}

impl ProtoConvertStruct {
    fn from_derive_input(
        name: Ident,
        attrs: &[Attribute],
        data: &DataStruct,
    ) -> darling::Result<Self> {
        let fields = Self::get_fields_with_attrs(data)?;
        let attrs = attrs.into();
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

                let attrs = ProtoConvertFieldAttrs::from(f.attrs.as_ref());
                Ok((ident, attrs))
            })
            .collect()
    }
}

impl ToTokens for ProtoConvertStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let source = self.attrs.source.as_ref();

        let from_proto_impl = {
            quote! {}
        };

        // let to_proto_impl = {
        //     quote! {}
        // };

        let expanded = quote! {
            impl ProtoConvert for #name {
                type ProtoStruct = ();

                // fn from_proto(p: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                //     #from_proto_impl
                // }

                // fn to_proto(&self) -> Self::ProtoStruct {
                //     #to_proto_impl
                // }
            }
        };
        tokens.extend(expanded);
    }
}

#[derive(Debug, FromMeta, Default)]
#[darling(default)]
struct ProtoConvertStructAttrs {
    source: Option<Path>,
}

impl From<&[Attribute]> for ProtoConvertStructAttrs {
    fn from(attrs: &[Attribute]) -> Self {
        let metas = find_proto_convert_meta(attrs);
        Self::from_list(&metas).unwrap_or_default()
    }
}

#[derive(Debug, FromMeta, Default)]
#[darling(default)]
struct ProtoConvertFieldAttrs {
    skip: bool,
    // with: Option<Path>,
}

impl ProtoConvertFieldAttrs {
    // TODO support skip
    fn impl_field_setter(&self, ident: &Ident) -> impl ToTokens {
        let proto_getter = Ident::new(&format!("{}", ident), Span::call_site());

        let setter = if self.skip {
            // Default setter for the skipped fields.
            quote! { Default::default() }
        } else {
            // Usual setter.
            quote! { ProtoConvert::from_proto(pb.#proto_getter().to_owned())? }
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

    // TODO
    // fn impl_field_getter(&self, ident: &Ident) -> impl ToTokens {
    //     let pb_setter = Ident::new(&format!("set_{}", ident), Span::call_site());
    //
    //     match (self.skip, &self.with) {
    //         // Usual getter.
    //         (false, None) => quote! {
    //             msg.#pb_setter(ProtobufConvert::to_pb(&self.#ident).into());
    //         },
    //         // Getter with the overridden Protobuf conversion.
    //         (false, Some(with)) => quote! {
    //             msg.#pb_setter(#with::to_pb(&self.#ident).into());
    //         },
    //         // Skipped getter does nothing.
    //         (true, _) => quote! {},
    //     }
    // }
}

impl From<&[Attribute]> for ProtoConvertFieldAttrs {
    fn from(attrs: &[Attribute]) -> Self {
        let metas = find_proto_convert_meta(attrs);
        Self::from_list(&metas).unwrap_or_default()
    }
}
