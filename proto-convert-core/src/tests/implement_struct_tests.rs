use crate::experimental::from_derive_input;
use crate::tests::assert_tokens_eq;
use quote::quote;
use syn::DeriveInput;

#[test]
fn implement_struct_primitives_test() {
    let fragment = quote! {
        #[proto_convert(source = "proto::Test")]
        struct Test {
            id: u32,
            valid: bool,
            opt_name: Option<String>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input(&input).unwrap();

    let expected = quote! {
        impl ProtoConvert<proto::Test> for Test {
            fn to_proto(&self) -> proto::Test {
                let mut proto = proto::Test::default();

                proto.set_id(ProtoConvert::to_proto(&self.id).into());
                proto.set_valid(ProtoConvert::to_proto(&self.valid).into());

                if let Some(value) = &self.opt_name {
                    proto.set_opt_name(ProtoConvert::to_proto(value).into());
                }

                proto
            }

            fn from_proto(proto: proto::Test) -> std::result::Result<Self, anyhow::Error> {
                let inner = Self {

                };
                Ok(inner)
            }
        }
    };

    let implemented = s.implement_proto_convert();

    assert_tokens_eq(&expected, &implemented);

    // std::stringify!(implemented);
}
