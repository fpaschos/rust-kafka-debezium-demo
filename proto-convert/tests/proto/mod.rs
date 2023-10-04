pub use entities::*;

pub mod entities {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub trait ProtoConvert: Sized {
    // Type of the protobuf clone of Self
    type ProtoStruct;

    // Struct -> ProtoStruct
    // fn to_proto(&self) -> Self::ProtoStruct;

    //ProtoStruct -> Struct
    // fn from_proto(pb: Self::ProtoStruct) -> Result<Self, anyhow::Error>;
}
