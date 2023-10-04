mod impl_proto_convert;

use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, Attribute};

const CONVERT_ATTRIBUTE: &str = "proto_convert";

#[proc_macro_derive(ProtoConvert, attributes(proto_convert))]
pub fn generate_proto_convert(input: TokenStream) -> TokenStream {
    impl_proto_convert::implement_proto_convert(input)
}

pub(crate) fn find_proto_convert_meta(attrs: &[Attribute]) -> Vec<NestedMeta> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident(CONVERT_ATTRIBUTE))
        .filter_map(|a| NestedMeta::parse_meta_list(a.to_token_stream()).ok())
        .flatten()
        .collect()
}
