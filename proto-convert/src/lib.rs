mod impl_proto_convert;
use proc_macro::TokenStream;

#[proc_macro_derive(ProtoConvert, attributes(proto_convert))]
pub fn generate_protobuf_convert(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
