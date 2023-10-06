use proto_convert::{derive::ProtoConvert, ProtoConvert};
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
#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::HierarchyEntity", oneof_field = "data")]
enum HierarchyEntity {
    FirstEntity(Entity),
    SecondEntity(NestedEntity),
}
