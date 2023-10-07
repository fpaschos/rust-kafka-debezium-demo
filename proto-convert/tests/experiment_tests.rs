use anyhow::Error;
use uuid::Uuid;

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
        todo!()
    }

    fn from_proto(proto: String) -> Result<Self, Error> {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub valid: bool,
    pub name: String,
}

impl ProtoConvert<proto::Entity> for Entity {
    fn from_proto(proto: proto::Entity) -> Result<Self, anyhow::Error> {
        let inner = Self {
            id: ProtoConvertPrimitive::from_proto(proto.id().to_owned())?,
            nonce: ProtoConvertPrimitive::from_proto(proto.nonce().to_owned())?,
            valid: ProtoConvertPrimitive::from_proto(proto.valid().to_owned())?,
            name: ProtoConvertPrimitive::from_proto(proto.name().to_owned())?,
        };
        Ok(inner)
    }
    fn to_proto(&self) -> proto::Entity {
        let mut msg = proto::Entity::default();
        msg.set_id(ProtoConvertPrimitive::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvertPrimitive::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvertPrimitive::to_proto(&self.valid).into());
        msg.set_name(ProtoConvertPrimitive::to_proto(&self.name).into());
        msg
    }
}

#[derive(Debug, Eq, PartialEq)]
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
    fn to_proto(&self) -> proto::EntityWithOptionals {
        let mut msg = proto::EntityWithOptionals::default();
        msg.set_id(ProtoConvertPrimitive::to_proto(&self.id).into());
        msg.set_nonce(ProtoConvertPrimitive::to_proto(&self.nonce).into());
        msg.set_valid(ProtoConvertPrimitive::to_proto(&self.valid).into());
        msg.set_name(ProtoConvertPrimitive::to_proto(&self.name).into());

        msg.set_opt_id(
            ProtoConvertPrimitive::to_proto(&self.opt_id.clone().unwrap_or_default()).into(),
        );
        msg.set_opt_nonce(
            ProtoConvertPrimitive::to_proto(&self.opt_nonce.clone().unwrap_or_default()).into(),
        );
        msg.set_opt_valid(
            ProtoConvertPrimitive::to_proto(&self.opt_valid.clone().unwrap_or_default()).into(),
        );
        msg.set_opt_name(
            ProtoConvertPrimitive::to_proto(&self.opt_name.clone().unwrap_or_default()).into(),
        );
        msg
    }
}

#[test]
fn test_roundtrip() {
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
