pub use entities::*;

pub mod entities {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub trait ProtoConvert: Sized {
    /// Type of the protobuf clone of Self
    type ProtoStruct;

    /// Struct -> ProtoStruct
    // fn to_proto(&self) -> Self::ProtoStruct;

    /// Consumes a proto [`Self::ProtoStruct`] and returns a [`Self`] struct
    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error>;
}

impl ProtoConvert for u32 {
    type ProtoStruct = Self;

    // fn to_proto(&self) -> Self::ProtoStruct {
    //     *self
    // }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for i32 {
    type ProtoStruct = Self;

    // fn to_proto(&self) -> Self::ProtoStruct {
    //     *self
    // }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for bool {
    type ProtoStruct = Self;

    // fn to_proto(&self) -> Self::ProtoStruct {
    //     *self
    // }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}

impl ProtoConvert for String {
    type ProtoStruct = Self;

    // fn to_proto(&self) -> Self::ProtoStruct {
    //     *self
    // }

    fn from_proto(proto: Self::ProtoStruct) -> Result<Self, anyhow::Error> {
        Ok(proto)
    }
}
