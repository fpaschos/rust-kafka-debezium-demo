use crate::find_proto_convert_meta;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
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
}

impl ToTokens for ProtoConvertEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let proto_struct = &self.attrs.source;

        let expanded = quote! {
            impl ProtoConvert for #name {
                type ProtoStruct = #proto_struct;

                 fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                    // #from_proto_impl
                    todo!();
                }

                fn to_proto(&self) -> Self::ProtoStruct {
                    // #to_proto_impl
                    todo!();
                }
            }
        };
        tokens.extend(expanded)
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct ProtoConvertEnumAttrs {
    source: Path,
    oneof_field: Ident,
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
