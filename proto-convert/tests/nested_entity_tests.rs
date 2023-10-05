use proto::ProtoConvert;
use proto_convert_derive::ProtoConvert;

mod proto;

#[derive(Debug, ProtoConvert, Eq, PartialEq)]
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
    pub token: String,
    pub inner: Entity,
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
        token: "nested_entity".into(),
        inner: entity,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
