use crate::proto::ProtoConvert;
use proto_convert::ProtoConvert;
use std::default::Default;
mod proto;

#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

#[test]
fn entity_round_trip() {
    let expected = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    let proto = proto::Entity {
        id: 1,
        nonce: 10,
        name: "Foo".to_string(),
        valid: true,
        ..Default::default()
    };

    let e = Entity::from_proto(proto).unwrap();
    assert_eq!(e, expected);
}
