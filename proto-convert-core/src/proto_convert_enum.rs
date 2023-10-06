use crate::{find_proto_convert_meta, SNAKE_CASE_ATTRIBUTE_VALUE};
use darling::FromMeta;
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, DataEnum, Fields, Path, Type, Variant};

#[derive(Debug)]
pub(crate) struct ProtoConvertEnum {
    pub name: Ident,
    pub attrs: ProtoConvertEnumAttrs,
    pub variants: Vec<EnumVariant>,
}

impl ProtoConvertEnum {
    pub(crate) fn from_derive_input(
        name: Ident,
        attrs: &[Attribute],
        data: &DataEnum,
    ) -> darling::Result<Self> {
        let attrs = ProtoConvertEnumAttrs::try_from(attrs)?;
        let variants: Vec<EnumVariant> = data
            .variants
            .iter()
            .map(EnumVariant::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name,
            attrs,
            variants,
        })
    }

    fn impl_proto_convert(&self) -> impl ToTokens {
        let name = &self.name;
        let proto_struct = &self.attrs.source;
        let one_of_field = &self.attrs.oneof_field;

        let from_proto_impl = quote! {};

        let to_proto_impl = {
            let match_arms = self.variants.iter().map(|variant| {
                let variant_name = &variant.name;
                let proto_variant_name = self.get_proto_variant_name(variant);

                let setter = Ident::new(&format!("set_{}", proto_variant_name), Span::call_site());
                quote! {
                     #name::#variant_name(value) => inner.#setter(value.to_proto()),
                }
            });

            quote! {
                let mut inner = Self::ProtoStruct::new();
                match self {
                    #( #match_arms )*
                }
                inner
            }
        };

        quote! {
            impl ProtoConvert for #name {
                type ProtoStruct = #proto_struct;

                 fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                    #from_proto_impl
                    todo!("ProtoConvert `from_proto(...)` not yet implemented for Enum items");
                }

                fn to_proto(&self) -> Self::ProtoStruct {
                    #to_proto_impl
                }
            }
        }
    }

    fn get_proto_variant_name(&self, variant: &EnumVariant) -> String {
        if let Some(rename_attr) = self.attrs.rename_variants.as_ref() {
            if rename_attr == SNAKE_CASE_ATTRIBUTE_VALUE {
                return variant.name.to_string().to_snake_case();
            } else {
                panic!(
                    "{}",
                    format!("Unknown attribute `rename_variants` = `{}`", rename_attr)
                );
            }
        }

        variant.name.to_string()
    }
}

impl ToTokens for ProtoConvertEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let proto_convert = self.impl_proto_convert();

        let expanded = quote! {
            #proto_convert
        };
        tokens.extend(expanded)
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct ProtoConvertEnumAttrs {
    source: Path,
    oneof_field: Ident,
    rename_variants: Option<String>,
}

impl TryFrom<&[Attribute]> for ProtoConvertEnumAttrs {
    type Error = darling::Error;

    fn try_from(attrs: &[Attribute]) -> Result<Self, Self::Error> {
        let meta = find_proto_convert_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing meta attribute `proto_convert`")
        })?;
        Self::from_meta(meta)
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariant {
    name: Ident,
    field_name: Path,
}

impl TryFrom<&Variant> for EnumVariant {
    type Error = darling::Error;

    fn try_from(value: &Variant) -> Result<Self, Self::Error> {
        let name = value.ident.clone();
        let field_name = match &value.fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return Err(darling::Error::unsupported_shape(
                        "Only unnamed variants in form `Foo(Bar)` are supported.",
                    ));
                }

                // Note: .first() here never fails
                match &fields.unnamed.first().unwrap().ty {
                    Type::Path(type_path) => Ok(type_path.path.clone()),
                    _ => Err(darling::Error::unsupported_shape(
                        "Only unnamed variants in form `Foo(Bar)` are supported.",
                    )),
                }
            }
            // Fields::Unit => {}
            _ => Err(darling::Error::unsupported_shape(
                "Only unnamed variants in form `Foo(Bar)` are supported.",
            )),
        }?;
        Ok(Self { name, field_name })
    }
}
