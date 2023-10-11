use crate::find_proto_convert_meta;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, Path};

/// Meta attributes for `struct` items
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub(crate) struct ProtoConvertStructAttrs {
    pub source: Option<Path>,
    /// Optional renaming of the struct fields before mapping to the proto entity.
    pub rename_all: Option<String>,
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

/// Meta attributes for `struct field` items
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub(crate) struct ProtoConvertFieldAttrs {
    /// Optional skipping struct field from proto serialization
    skip: bool,
    /// Optional renaming of a single struct field before mapping to the proto entity.
    rename: Option<String>,
    // with: Option<Path>,
}

impl ProtoConvertFieldAttrs {
    // TODO support skip
    pub(crate) fn impl_struct_field_setter(&self, ident: &Ident) -> TokenStream {
        let field_name = self.get_proto_field_name(ident, None);
        let proto_getter = Ident::new(&field_name, Span::call_site());

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

    pub(crate) fn impl_struct_field_getter(&self, ident: &Ident) -> TokenStream {
        // Handles rename
        let field_name = self.get_proto_field_name(ident, Some('_'));
        let proto_setter = Ident::new(&format!("set_{}", field_name), Span::call_site());
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

    /// Returns a struct field name given an identifier and a rename field attribute.
    /// remove_last_char_if is used in cases that we want to remove special characters such as '_'
    fn get_proto_field_name(&self, field: &Ident, remove_last_char_if: Option<char>) -> String {
        if let Some(rename) = self.rename.as_ref() {
            if let Some(c) = remove_last_char_if {
                let mut rename_rev = rename.chars().rev().peekable();
                if rename_rev.peek().copied() == Some(c) {
                    return rename[..rename.len() - 1].to_string();
                }
            }
            return rename.to_string();
        }
        return field.to_string();
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
