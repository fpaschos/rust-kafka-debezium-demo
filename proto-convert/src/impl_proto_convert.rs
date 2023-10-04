use proc_macro::TokenStream;
use quote::quote;

pub fn implement_protobuf_convert(input: TokenStream) -> TokenStream {
    let input = ProtoConvert::from_derive_input(&syn::parse(input).unwrap())
        .unwrap_or_else(|e| panic!("ProtoConvert: {}", e));
    let tokens = quote! {#input};
    tokens.into()
}

enum ProtoConvert {
    Struct(ProtoConvertStruct),
}

struct ProtoConvertStruct {}
