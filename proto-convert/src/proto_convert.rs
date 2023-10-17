use anyhow::Error;
use std::str::FromStr;
use uuid::Uuid;

pub trait ProtoScalar: Sized + private::Sealed {
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
impl ProtoScalar for u32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoScalar for i32 {
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl ProtoScalar for bool {
    fn has_value(&self) -> bool {
        *self
    }
}

impl ProtoScalar for String {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoScalar for Vec<u8> {
    fn has_value(&self) -> bool {
        !self.is_empty()
    }
}

impl ProtoScalar for f32 {
    fn has_value(&self) -> bool {
        *self != 0f32
    }
}

impl ProtoScalar for f64 {
    fn has_value(&self) -> bool {
        *self != 0f64
    }
}

pub trait ProtoConvertScalar<P: ProtoScalar>: Sized {
    fn to_scalar(&self) -> P;

    fn from_scalar(proto: P) -> Result<Self, anyhow::Error>;
}

pub trait ProtoConvert
where
    Self: Sized,
{
    type ProtoStruct;
    /// Converts a reference of [`Self`] struct to proto [`Self::ProtoStruct`]
    fn to_proto(&self) -> Self::ProtoStruct;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error>;
}

impl ProtoConvertScalar<u32> for u32 {
    fn to_scalar(&self) -> u32 {
        *self
    }

    fn from_scalar(proto: u32) -> Result<Self, Error> {
        Ok(proto)
    }
}

impl ProtoConvertScalar<i32> for i32 {
    fn to_scalar(&self) -> i32 {
        *self
    }

    fn from_scalar(proto: i32) -> Result<Self, Error> {
        Ok(proto)
    }
}

impl ProtoConvertScalar<bool> for bool {
    fn to_scalar(&self) -> bool {
        *self
    }

    fn from_scalar(proto: bool) -> Result<Self, Error> {
        Ok(proto)
    }
}

impl ProtoConvertScalar<String> for String {
    fn to_scalar(&self) -> String {
        self.clone()
    }

    fn from_scalar(proto: String) -> Result<Self, Error> {
        Ok(proto)
    }
}

impl ProtoConvertScalar<String> for Uuid {
    fn to_scalar(&self) -> String {
        self.to_string()
    }

    fn from_scalar(proto: String) -> Result<Self, Error> {
        let res = Uuid::from_str(&proto)?;
        Ok(res)
    }
}

// Implementation from https://stackoverflow.com/questions/65268226/rust-deserialization-converting-vector-of-bytes-to-hashset-of-uuid
impl ProtoConvertScalar<Vec<u8>> for Uuid {
    fn to_scalar(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(16);
        res.extend_from_slice(self.as_bytes());
        res
    }

    fn from_scalar(proto: Vec<u8>) -> Result<Self, Error> {
        Ok(Uuid::from_slice(&proto)?)
    }
}
