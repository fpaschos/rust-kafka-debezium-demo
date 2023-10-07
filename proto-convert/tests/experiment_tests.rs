use anyhow::Error;
use std::str::FromStr;
use uuid::Uuid;
/// Fully expanded and manual experiments (these used to build the macros and the library traits synergy)
mod proto;

pub trait ProtoConvert<Proto>
where
    Self: Sized,
{
    /// Type of the protobuf clone of Self

    /// Converts a reference of [`Self`] struct to proto [`Self::ProtoStruct`]
    fn to_proto(&self) -> Proto;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct
    fn from_proto(proto: Proto) -> Result<Self, anyhow::Error>;
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
///TODO see https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed for sealed trait
pub trait ProtoConvertPrimitive<ProtoRepr>: Sized {
    /// Converts a reference of [`Self`] struct to proto [`Self::Proto`]
    fn to_proto(&self) -> ProtoRepr;

    /// Consumes a proto [`Self::Proto`] and returns a [`Self`] struct
    fn from_proto(proto: ProtoRepr) -> Result<Self, anyhow::Error>;
}

impl ProtoConvertPrimitive<u32> for u32 {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvertPrimitive<i32> for i32 {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvertPrimitive<bool> for bool {
    fn to_proto(&self) -> Self {
        *self
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvertPrimitive<String> for String {
    fn to_proto(&self) -> Self {
        self.clone()
    }

    fn from_proto(proto: Self) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvertPrimitive<String> for Uuid {
    fn to_proto(&self) -> String {
        self.to_string()
    }

    fn from_proto(proto: String) -> Result<Self, Error> {
        Ok(Uuid::from_str(&proto)?)
    }
}

// impl ProtoConvertPrimitive<Vec> for Uuid {
//     fn to_proto(&self) -> String {
//         self.to_string()
//     }
//
//     fn from_proto(proto: String) -> Result<Self, Error> {
//         Ok(Uuid::from_str(&proto)?)
//     }
// }

#[derive(Debug, PartialEq)]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

impl ProtoConvert<proto::Entity> for Entity {
    fn to_proto(&self) -> proto::Entity {
        let mut msg = proto::Entity::default();
        msg.set_id(ProtoConvertPrimitive::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvertPrimitive::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvertPrimitive::to_proto(&self.valid).into());
        msg.set_name(ProtoConvertPrimitive::to_proto(&self.name).into());
        msg
    }
    fn from_proto(proto: proto::Entity) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvertPrimitive::from_proto(proto.id().to_owned())?,
            nonce: ProtoConvertPrimitive::from_proto(proto.nonce().to_owned())?,
            valid: ProtoConvertPrimitive::from_proto(proto.valid().to_owned())?,
            name: ProtoConvertPrimitive::from_proto(proto.name().to_owned())?,
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
        msg.set_id(ProtoConvertPrimitive::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvertPrimitive::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvertPrimitive::to_proto(&self.valid).into());
        msg.set_name(ProtoConvertPrimitive::to_proto(&self.name).into());

        // Only if there is value other default
        if let Some(value) = &self.opt_id {
            msg.set_opt_id(ProtoConvertPrimitive::to_proto(value).into());
        }

        // Only if there is value other default
        if let Some(value) = &self.opt_nonce {
            msg.set_opt_nonce(ProtoConvertPrimitive::to_proto(value).into());
        }
        if let Some(value) = &self.opt_valid {
            msg.set_opt_valid(ProtoConvertPrimitive::to_proto(value).into());
        }

        if let Some(value) = &self.opt_name {
            msg.set_opt_name(ProtoConvertPrimitive::to_proto(value).into());
        }
        msg
    }
    fn from_proto(proto: proto::EntityWithOptionals) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvertPrimitive::from_proto(proto.id().to_owned())?,
            nonce: ProtoConvertPrimitive::from_proto(proto.nonce().to_owned())?,
            valid: ProtoConvertPrimitive::from_proto(proto.valid().to_owned())?,
            name: ProtoConvertPrimitive::from_proto(proto.name().to_owned())?,
            // Special case for options
            opt_id: {
                let v = proto.opt_id().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_nonce: {
                let v = proto.opt_nonce().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_valid: {
                let v = proto.opt_valid().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_proto(v)?)
                } else {
                    None
                }
            },
            opt_name: {
                let v = proto.opt_name().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_proto(v)?)
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
        msg.set_uuid_str(ProtoConvertPrimitive::to_proto(&self.uuid_str).into());

        // Only if there is value other default
        if let Some(value) = &self.opt_uuid_str {
            msg.set_opt_uuid_str(ProtoConvertPrimitive::to_proto(value).into());
        }

        msg
    }
    fn from_proto(proto: proto::EntityUuids) -> Result<Self, anyhow::Error> {
        let inner = Self {
            uuid_str: ProtoConvertPrimitive::from_proto(proto.uuid_str().to_owned())?,
            opt_uuid_str: {
                let v = proto.opt_uuid_str().to_owned();
                if ProtoPrimitiveValue::has_value(&v) {
                    Some(ProtoConvertPrimitive::from_proto(v)?)
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
