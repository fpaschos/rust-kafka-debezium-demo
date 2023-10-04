mod impl_proto_convert;

use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, Attribute, Meta};

const CONVERT_ATTRIBUTE: &str = "proto_convert";

#[proc_macro_derive(ProtoConvert, attributes(proto_convert))]
pub fn generate_proto_convert(input: TokenStream) -> TokenStream {
    impl_proto_convert::implement_proto_convert(input)
}

pub(crate) fn find_proto_convert_meta(attrs: &[Attribute]) -> Option<&Meta> {
    attrs
        .iter()
        .find(|a| a.path().is_ident(CONVERT_ATTRIBUTE))
        .map(|a| &a.meta)
}
