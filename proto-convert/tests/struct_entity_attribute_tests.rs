use proto_convert::derive::ProtoConvert;
use proto_convert::{ProtoConvert, ProtoConvertPrimitive};
use std::default::Default;
mod proto;

#[derive(Debug, Clone, Copy, Eq, PartialEq, ProtoConvert)]
#[proto_convert(
    source = "proto::EntityStatus",
    enumeration,
    rename_variants = "STREAMING_SNAKE_CASE"
)]
pub enum EntityStatus {
    StatusA,
    StatusB,
    StatusC,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ProtoConvert)]
#[proto_convert(
    source = "proto::EntityType",
    enumeration,
    rename_variants = "STREAMING_SNAKE_CASE"
)]
pub enum EntityType {
    TypeA,
    TypeB,
    TypeC,
}

#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    #[proto_convert(skip)]
    pub name: String,
    pub status: EntityStatus,
    #[proto_convert(rename = "type_")]
    pub r#type: EntityType,
}

#[test]
fn entity_round_trip() {
    let mut original = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Bar".to_string(),
        status: EntityStatus::StatusA,
        r#type: EntityType::TypeC,
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    original.name = Default::default(); // Name is skipped so default
    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let mut original = proto::Entity {
        id: 1,
        nonce: 10,
        name: "Foo".to_string(),
        valid: true,
        ..Default::default()
    };

    let e = Entity::from_proto(original.clone()).unwrap();
    let tested = e.to_proto();

    original.name = Default::default(); // Name is skipped so default
    assert_eq!(tested, original);
}
