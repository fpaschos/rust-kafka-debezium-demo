use crate::proto::EntityCommon;
use proto_convert::derive::ProtoConvert;
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
    let original = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let original = proto::Entity {
        id: 1,
        nonce: 10,
        name: "Foo".to_string(),
        valid: true,
        ..Default::default()
    };

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    assert_eq!(tested, original);
}

#[test]
fn test_use_from_module() {
    // Just use the entity from another module
    let e = EntityCommon { id: 10 };
    let p = e.to_proto();
    let _ = EntityCommon::from_proto(p).unwrap();
}
