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

impl ProtoPrimitiveValue for bool {
    fn has_value(&self) -> bool {
        *self
    }
}

impl ProtoPrimitiveValue for String {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

pub trait ProtoConvert: Sized {
    /// Type of the protobuf clone of Self
    type ProtoStruct;

    /// Converts a reference of [`Self`] struct to proto [`Self::ProtoStruct`]
    fn to_proto(&self) -> Self::ProtoStruct;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error>;
}

impl ProtoConvert for u32 {
    type ProtoStruct = Self;

    fn to_proto(&self) -> Self::ProtoStruct {
        *self
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for i32 {
    type ProtoStruct = Self;

    fn to_proto(&self) -> Self::ProtoStruct {
        *self
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for bool {
    type ProtoStruct = Self;

    fn to_proto(&self) -> Self::ProtoStruct {
        *self
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for String {
    type ProtoStruct = Self;

    fn to_proto(&self) -> Self::ProtoStruct {
        self.clone()
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

// TODO remove
impl<T: ProtoConvert + Default + Clone + PartialEq> ProtoConvert for Option<T> {
    type ProtoStruct = T;

    fn to_proto(&self) -> Self::ProtoStruct {
        match self {
            None => Default::default(),
            Some(value) => value.clone(),
        }
    }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        if proto == Self::ProtoStruct::default() {
            // TODO Remove because of this expensive use of default()
            Ok(None)
        } else {
            Ok(Some(proto))
        }
    }
}
