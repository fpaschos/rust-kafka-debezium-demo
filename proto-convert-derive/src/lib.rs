use proc_macro::TokenStream;
use proto_convert_core::proto_convert::implement_proto_convert;

#[proc_macro_derive(ProtoConvert, attributes(proto_convert))]
pub fn generate_proto_convert(input: TokenStream) -> TokenStream {
    implement_proto_convert(input.into()).into()
}
