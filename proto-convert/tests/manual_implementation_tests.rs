use proto_convert::{ProtoConvert, ProtoConvertPrimitive, ProtoPrimitive};
use protobuf::Enum;
use uuid::Uuid;

/// Fully expanded and manual experiments (these used to build the macros and the library traits synergy)
mod proto;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EntityStatus {
    StatusA,
    StatusB,
    StatusC,
}

// Example of manual implementation for enumeration to primitive
impl ProtoConvert for EntityStatus {
    type ProtoStruct = proto::EntityStatus;
    fn to_proto(&self) -> Self::ProtoStruct {
        match self {
            Self::StatusA => proto::EntityStatus::STATUS_A,
            Self::StatusB => proto::EntityStatus::STATUS_B,
            Self::StatusC => proto::EntityStatus::STATUS_C,
        }
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        match proto {
            proto::EntityStatus::STATUS_A => Ok(Self::StatusA),
            proto::EntityStatus::STATUS_B => Ok(Self::StatusB),
            proto::EntityStatus::STATUS_C => Ok(Self::StatusC),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
    pub status: EntityStatus,
}

impl ProtoConvert for Entity {
    type ProtoStruct = proto::Entity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::Entity::default();
        proto.set_id(ProtoConvertPrimitive::to_primitive(&self.id));
        proto.set_nonce(ProtoConvertPrimitive::to_primitive(&self.nonce));
        proto.set_valid(ProtoConvertPrimitive::to_primitive(&self.valid));
        proto.set_name(ProtoConvertPrimitive::to_primitive(&self.name));
        // Special case for enum
        proto.set_status(ProtoConvert::to_proto(&self.status));
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvertPrimitive::from_primitive(proto.id().to_owned())?,
            nonce: ProtoConvertPrimitive::from_primitive(proto.nonce().to_owned())?,
            valid: ProtoConvertPrimitive::from_primitive(proto.valid().to_owned())?,
            name: ProtoConvertPrimitive::from_primitive(proto.name().to_owned())?,
            // Special case for enum
            status: ProtoConvert::from_proto(proto.status().to_owned())?,
        };
        Ok(inner)
    }
}

#[derive(Debug, PartialEq)]
struct EntityWithOptionals {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
    pub opt_id: Option<u32>,
    pub opt_nonce: Option<i32>,
    pub opt_valid: Option<bool>,
    pub opt_name: Option<String>,
    pub opt_status: Option<EntityStatus>,
}

impl ProtoConvert for EntityWithOptionals {
    type ProtoStruct = proto::EntityWithOptionals;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::EntityWithOptionals::default();
        proto.set_id(ProtoConvertPrimitive::to_primitive(&self.id));
        proto.set_nonce(ProtoConvertPrimitive::to_primitive(&self.nonce));
        proto.set_valid(ProtoConvertPrimitive::to_primitive(&self.valid));
        proto.set_name(ProtoConvertPrimitive::to_primitive(&self.name));

        // Only if there is value other default
        if let Some(value) = &self.opt_id {
            proto.set_opt_id(ProtoConvertPrimitive::to_primitive(value));
        }

        // Only if there is value other default
        if let Some(value) = &self.opt_nonce {
            proto.set_opt_nonce(ProtoConvertPrimitive::to_primitive(value));
        }

        if let Some(value) = &self.opt_valid {
            proto.set_opt_valid(ProtoConvertPrimitive::to_primitive(value));
        }

        if let Some(value) = &self.opt_name {
            proto.set_opt_name(ProtoConvertPrimitive::to_primitive(value));
        }

        if let Some(value) = &self.opt_status {
            proto.set_opt_status(ProtoConvert::to_proto(value));
        }
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvertPrimitive::from_primitive(proto.id().to_owned())?,
            nonce: ProtoConvertPrimitive::from_primitive(proto.nonce().to_owned())?,
            valid: ProtoConvertPrimitive::from_primitive(proto.valid().to_owned())?,
            name: ProtoConvertPrimitive::from_primitive(proto.name().to_owned())?,
            // Special case for options
            opt_id: {
                let v = proto.opt_id().to_owned();
                if ProtoPrimitive::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_primitive(v)?)
                } else {
                    None
                }
            },
            opt_nonce: {
                let v = proto.opt_nonce().to_owned();
                if ProtoPrimitive::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_primitive(v)?)
                } else {
                    None
                }
            },
            opt_valid: {
                let v = proto.opt_valid().to_owned();
                if ProtoPrimitive::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_primitive(v)?)
                } else {
                    None
                }
            },
            opt_name: {
                let v = proto.opt_name().to_owned();
                if ProtoPrimitive::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_primitive(v)?)
                } else {
                    None
                }
            },
            // Special case for enumerations
            opt_status: {
                let v = proto.opt_status().to_owned();
                // convert enum value to i32 in order to check ProtoPrimitive value
                if ProtoPrimitive::has_value(&v.value()) {
                    Some(ProtoConvert::from_proto(v)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

// Example of manual implementation for uuid to primitive

#[derive(Debug, PartialEq)]
pub struct EntityUuids {
    uuid_str: Uuid,
    opt_uuid_str: Option<Uuid>,
    // uuid_3: Uuid,
    // uuid_4: Option<Uuid>,
}

impl ProtoConvert for EntityUuids {
    type ProtoStruct = proto::EntityUuids;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::EntityUuids::default();
        proto.set_uuid_str(ProtoConvertPrimitive::to_primitive(&self.uuid_str));

        // Only if there is value other default
        if let Some(value) = &self.opt_uuid_str {
            proto.set_opt_uuid_str(ProtoConvertPrimitive::to_primitive(value));
        }

        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            uuid_str: ProtoConvertPrimitive::from_primitive(proto.uuid_str().to_owned())?,
            opt_uuid_str: {
                let v = proto.opt_uuid_str().to_owned();
                if ProtoPrimitive::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_primitive(v)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NestedEntity {
    first: Entity,
    second: Option<Entity>,
}

impl ProtoConvert for NestedEntity {
    type ProtoStruct = proto::NestedEntity;
    fn to_proto(&self) -> Self::ProtoStruct {
        let mut proto = proto::NestedEntity::default();
        proto.set_first(ProtoConvert::to_proto(&self.first).into());
        // Only if there is value other default
        if let Some(value) = &self.second {
            proto.set_second(ProtoConvert::to_proto(value).into());
        }
        proto
    }
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        let inner = Self {
            first: ProtoConvert::from_proto(proto.first().to_owned())?,
            second: {
                if proto.has_second() {
                    Some(ProtoConvert::from_proto(proto.second().to_owned())?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

#[test]
fn entity_test_round_trip() {
    let original = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        status: EntityStatus::StatusC,
    };

    let p = original.to_proto();
    let tested = Entity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn test_entity_with_optionals_round_trips() {
    let original = EntityWithOptionals {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        opt_id: None,
        opt_nonce: None,
        opt_valid: None,
        opt_name: None,
        opt_status: None,
    };

    let p = original.to_proto();
    let tested = EntityWithOptionals::from_proto(p).unwrap();

    assert_eq!(tested, original);

    let original = EntityWithOptionals {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        opt_id: Some(1),
        opt_nonce: Some(2),
        opt_valid: Some(true),
        opt_name: Some("Foo1".into()),
        opt_status: Some(EntityStatus::StatusC),
    };

    let p = original.to_proto();
    let tested = EntityWithOptionals::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn test_entity_uuids_round_trips() {
    // Test with value
    let original = EntityUuids {
        uuid_str: Uuid::new_v4(),
        opt_uuid_str: Some(Uuid::new_v4()),
    };

    let p = original.to_proto();
    let tested = EntityUuids::from_proto(p).unwrap();
    assert_eq!(tested, original);

    // Test with none
    let original = EntityUuids {
        uuid_str: Uuid::new_v4(),
        opt_uuid_str: None,
    };

    let p = original.to_proto();
    let tested = EntityUuids::from_proto(p).unwrap();
    assert_eq!(tested, original);
}

#[test]
fn nested_entity_test_round_trips() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        status: EntityStatus::StatusB,
    };

    let original = NestedEntity {
        first: entity.clone(),
        second: None,
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);

    let original = NestedEntity {
        first: entity.clone(),
        second: Some(entity.clone()),
    };

    let p = original.to_proto();
    let tested = NestedEntity::from_proto(p).unwrap();

    assert_eq!(tested, original);
}
