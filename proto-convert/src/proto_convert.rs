pub trait ProtoPrimitive: Sized + private::Sealed {
    fn has_value(&self) -> bool;
}

mod private {
    // see https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed for sealed trait
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for u64 {}
    impl Sealed for i64 {}
    impl Sealed for f64 {}
    impl Sealed for f32 {}
    impl Sealed for bool {}
    impl Sealed for String {}
    impl Sealed for Vec<u8> {}
}
impl ProtoPrimitive for u32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoPrimitive for i32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoPrimitive for bool {
    fn has_value(&self) -> bool {
        *self
    }
}

impl ProtoPrimitive for String {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoPrimitive for Vec<u8> {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoPrimitive for f32 {
    fn has_value(&self) -> bool {
        *self != 0f32
    }
}

impl ProtoPrimitive for f64 {
    fn has_value(&self) -> bool {
        *self != 0f64
    }
}

trait ProtoConvertPrimitive<P: ProtoPrimitive>: Sized {
    fn to_primitive(&self) -> P;

    fn from_primitive(proto: P) -> Result<Self, anyhow::Error>;
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
