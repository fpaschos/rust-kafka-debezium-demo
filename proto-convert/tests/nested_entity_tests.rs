use proto_convert::derive::ProtoConvert;
use proto_convert::{ProtoConvert, ProtoConvertPrimitive};

mod proto;

#[derive(Debug, Clone, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::NestedEntity")]
struct NestedEntity {
    pub first: Entity,
    pub second: Entity,
}

#[test]
fn nested_entity_round_trip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    let original = NestedEntity {
        first: entity.clone(),
        second: entity,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
