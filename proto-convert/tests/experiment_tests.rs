use anyhow::Error;
use std::str::FromStr;
use uuid::Uuid;
/// Fully expanded and manual experiments (these used to build the macros and the library traits synergy)
mod proto;

pub trait ProtoConvert<ProtoRepr>
where
    Self: Sized,
{
    /// Converts a reference of [`Self`] struct to proto [`Self::ProtoStruct`]
    fn to_proto(&self) -> ProtoRepr;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct
    fn from_proto(proto: ProtoRepr) -> Result<Self, anyhow::Error>;
}

///TODO see https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed for sealed trait
pub trait ProtoPrimitiveValue: Sized {
    fn has_value(&self) -> bool;
}
impl ProtoPrimitiveValue for u32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoPrimitiveValue for i32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoPrimitiveValue for String {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoPrimitiveValue for bool {
    fn has_value(&self) -> bool {
        *self
    }
}

impl ProtoConvert<u32> for u32 {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert<i32> for i32 {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert<bool> for bool {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert<String> for String {
    fn to_proto(&self) -> Self {
        self.clone()
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert<String> for Uuid {
    fn to_proto(&self) -> String {
        self.to_string()
    }

    fn from_proto(proto: String) -> Result<Self, Error> {
        Ok(Uuid::from_str(&proto)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

impl ProtoConvert<proto::Entity> for Entity {
    fn to_proto(&self) -> proto::Entity {
        let mut msg = proto::Entity::default();
        msg.set_id(ProtoConvert::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvert::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvert::to_proto(&self.valid).into());
        msg.set_name(ProtoConvert::to_proto(&self.name).into());
        msg
    }
    fn from_proto(proto: proto::Entity) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvert::from_proto(proto.id().to_owned())?,
            nonce: ProtoConvert::from_proto(proto.nonce().to_owned())?,
            valid: ProtoConvert::from_proto(proto.valid().to_owned())?,
            name: ProtoConvert::from_proto(proto.name().to_owned())?,
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
}

impl ProtoConvert<proto::EntityWithOptionals> for EntityWithOptionals {
    fn to_proto(&self) -> proto::EntityWithOptionals {
        let mut msg = proto::EntityWithOptionals::default();
        msg.set_id(ProtoConvert::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvert::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvert::to_proto(&self.valid).into());
        msg.set_name(ProtoConvert::to_proto(&self.name).into());

        // Only if there is value other default
        if let Some(value) = &self.opt_id {
            msg.set_opt_id(ProtoConvert::to_proto(value).into());
        }

        // Only if there is value other default
        if let Some(value) = &self.opt_nonce {
            msg.set_opt_nonce(ProtoConvert::to_proto(value).into());
        }
        if let Some(value) = &self.opt_valid {
            msg.set_opt_valid(ProtoConvert::to_proto(value).into());
        }

        if let Some(value) = &self.opt_name {
            msg.set_opt_name(ProtoConvert::to_proto(value).into());
        }
        msg
    }
    fn from_proto(proto: proto::EntityWithOptionals) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvert::from_proto(proto.id().to_owned())?,
            nonce: ProtoConvert::from_proto(proto.nonce().to_owned())?,
            valid: ProtoConvert::from_proto(proto.valid().to_owned())?,
            name: ProtoConvert::from_proto(proto.name().to_owned())?,
            // Special case for options
            opt_id: {
                let v = proto.opt_id().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvert::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_nonce: {
                let v = proto.opt_nonce().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvert::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_valid: {
                let v = proto.opt_valid().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvert::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_name: {
                let v = proto.opt_name().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvert::from_proto(v)?)
                } else {
                    None
                }
            },
        };
        Ok(inner)
    }
}

#[derive(Debug, PartialEq)]
pub struct EntityUuids {
    uuid_str: Uuid,
    opt_uuid_str: Option<Uuid>,
    // uuid_3: Uuid,
    // uuid_4: Option<Uuid>,
}

impl ProtoConvert<proto::EntityUuids> for EntityUuids {
    fn to_proto(&self) -> proto::EntityUuids {
        let mut msg = proto::EntityUuids::default();
        msg.set_uuid_str(ProtoConvert::to_proto(&self.uuid_str).into());

        // Only if there is value other default
        if let Some(value) = &self.opt_uuid_str {
            msg.set_opt_uuid_str(ProtoConvert::to_proto(value).into());
        }

        msg
    }
    fn from_proto(proto: proto::EntityUuids) -> Result<Self, anyhow::Error> {
        let inner = Self {
            uuid_str: ProtoConvert::from_proto(proto.uuid_str().to_owned())?,
            opt_uuid_str: {
                let v = proto.opt_uuid_str().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvert::from_proto(v)?)
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

impl ProtoConvert<proto::NestedEntity> for NestedEntity {
    fn to_proto(&self) -> proto::NestedEntity {
        let mut msg = proto::NestedEntity::default();
        msg.set_first(ProtoConvert::to_proto(&self.first).into());
        // Only if there is value other default
        if let Some(value) = &self.second {
            msg.set_second(ProtoConvert::to_proto(value).into());
        }
        msg
    }
    fn from_proto(proto: proto::NestedEntity) -> Result<Self, anyhow::Error> {
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
fn entity_test_roundtrip() {
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
fn test_entity_with_optionals_roundtrip() {
    let original = EntityWithOptionals {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
        opt_id: None,
        opt_nonce: None,
        opt_valid: None,
        opt_name: None,
    };

    let p = original.to_proto();
    let tested = EntityWithOptionals::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn test_entity_uuids_roundtrips() {
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
fn nested_entity_test_roundtrip() {
    let entity = Entity {
        id: 1,
        nonce: 10,
        valid: true,
        name: "Foo".into(),
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
