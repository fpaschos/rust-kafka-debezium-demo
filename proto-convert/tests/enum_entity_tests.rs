use crate::proto::hierarchy_entity::Data;
use anyhow::Error;
use proto_convert::{derive::ProtoConvert, ProtoConvert};

mod proto;
#[derive(Debug, Clone, ProtoConvert, PartialEq)]
#[proto_convert(source = "proto::Entity")]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

#[derive(Debug, ProtoConvert, PartialEq)]
#[proto_convert(source = "proto::NestedEntity")]
struct NestedEntity {
    pub first: Entity,
    pub second: Entity,
}
#[derive(Debug, ProtoConvert, PartialEq)]
#[proto_convert(
    source = "proto::HierarchyEntity",
    oneof_field = "data",
    rename_variants = "snake_case"
)]
enum HierarchyEntity {
    FirstEntity(Entity),
    SecondEntity(NestedEntity),
}

#[test]
fn hierarchy_entity_round_trip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    // let nested = NestedEntity {
    //     token: "nested_entity".into(),
    //     inner: entity,
    // };

    let original = HierarchyEntity::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

// Just for reference purposes implement the interface manually
#[derive(Debug, PartialEq)]
enum HierarchyEntityManual {
    FirstEntity(Entity),
    SecondEntity(NestedEntity),
}
impl ProtoConvert for HierarchyEntityManual {
    type ProtoStruct = proto::HierarchyEntity;

    fn to_proto(&self) -> Self::ProtoStruct {
        let mut inner = Self::ProtoStruct::new();
        match self {
            HierarchyEntityManual::FirstEntity(value) => inner.set_first_entity(value.to_proto()),
            HierarchyEntityManual::SecondEntity(value) => inner.set_second_entity(value.to_proto()),
        }
        inner
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, Error> {
        match proto.data {
            Some(Data::FirstEntity(v)) => {
                Entity::from_proto(v).map(HierarchyEntityManual::FirstEntity)
            }
            Some(Data::SecondEntity(v)) => {
                NestedEntity::from_proto(v).map(HierarchyEntityManual::SecondEntity)
            }

            None => Err(anyhow::anyhow!(
                "Failed to convert HierarchyEntityManual from protobuf"
            )),
        }
    }
}

#[test]
fn manual_hierarchy_entity_round_trip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
    };

    // let nested = NestedEntity {
    //     token: "nested_entity".into(),
    //     inner: entity,
    // };

    let original = HierarchyEntityManual::FirstEntity(entity);

    let p = original.to_proto();
    let tested = HierarchyEntityManual::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
