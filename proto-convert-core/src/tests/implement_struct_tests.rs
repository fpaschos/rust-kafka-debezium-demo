use crate::tests::{assert_tokens_eq, from_derive_input_struct};
use quote::quote;
use syn::DeriveInput;

#[test]
fn implement_struct_scalar_types_test() {
    let fragment = quote! {
        #[proto_convert(source = "proto::Test")]
        struct Test {
            id: u32,
            valid: bool,
            opt_name: Option<String>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input_struct(&input).unwrap();

    let expected = quote! {
        impl ProtoConvert for Test {
            type ProtoStruct = proto::Test;
            fn to_proto(&self) -> Self::ProtoStruct {
                let mut proto = proto::Test::default();

                proto.set_id(ProtoConvertScalar::to_scalar(&self.id).into());
                proto.set_valid(ProtoConvertScalar::to_scalar(&self.valid).into());

                if let Some(value) = &self.opt_name {
                    proto.set_opt_name(ProtoConvertScalar::to_scalar(value).into());
                }

                proto
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                let inner = Self {
                    id: ProtoConvertScalar::from_scalar(proto.id().to_owned())?,
                    valid: ProtoConvertScalar::from_scalar(proto.valid().to_owned())?,
                    opt_name: {
                        let value = proto.opt_name().to_owned();
                        if ProtoScalar::has_value(&value) {
                            Some(ProtoConvertScalar::from_scalar(value)?)
                        } else {
                            None
                        }
                    },
                };
                Ok(inner)
            }
        }
    };

    let actual = s.implement_proto_convert();
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn implement_struct_non_scalar_types_test() {
    let fragment = quote! {
        #[proto_convert(source = "proto::Test")]
        struct Test {
            entity: Entity,
            opt_entity: Option<Entity>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input_struct(&input).unwrap();

    let expected = quote! {
        impl ProtoConvert for Test {
            type ProtoStruct = proto::Test;
            fn to_proto(&self) -> Self::ProtoStruct {
                let mut proto = proto::Test::default();

                proto.set_entity(ProtoConvert::to_proto(&self.entity).into());

                if let Some(value) = &self.opt_entity {
                    proto.set_opt_entity(ProtoConvert::to_proto(value).into());
                }

                proto
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                let inner = Self {
                    entity: ProtoConvert::from_proto(proto.entity().to_owned())?,
                    opt_entity: {
                        let value = proto.opt_entity().to_owned();
                        if proto.has_opt_entity() {
                            Some(ProtoConvert::from_proto(value)?)
                        } else {
                            None
                        }
                    },
                };
                Ok(inner)
            }
        }
    };

    let actual = s.implement_proto_convert();
    assert_tokens_eq(&expected, &actual);
}

#[test]
fn implement_struct_rename_attributes_test() {
    let fragment = quote! {
        #[proto_convert(source = "proto::Test")]
        struct Test {
            #[proto_convert(rename = "type_")]
            r#type: Entity,
            #[proto_convert(rename = "other_name")]
            opt_entity: Option<Entity>,
        }
    };

    let input = syn::parse2::<DeriveInput>(fragment.into()).unwrap();

    let s = from_derive_input_struct(&input).unwrap();

    let expected = quote! {
        impl ProtoConvert for Test {
            type ProtoStruct = proto::Test;
            fn to_proto(&self) -> Self::ProtoStruct {
                let mut proto = proto::Test::default();

                proto.set_type(ProtoConvert::to_proto(&self.r#type).into());

                if let Some(value) = &self.opt_entity {
                    proto.set_other_name(ProtoConvert::to_proto(value).into());
                }

                proto
            }

            fn from_proto(proto: Self::ProtoStruct) -> std::result::Result<Self, anyhow::Error> {
                let inner = Self {
                    r#type: ProtoConvert::from_proto(proto.type_().to_owned())?,
                    opt_entity: {
                        let value = proto.other_name().to_owned();
                        if proto.has_other_name() {
                            Some(ProtoConvert::from_proto(value)?)
                        } else {
                            None
                        }
                    },
                };
                Ok(inner)
            }
        }
    };

    let actual = s.implement_proto_convert();
    assert_tokens_eq(&expected, &actual);
}
