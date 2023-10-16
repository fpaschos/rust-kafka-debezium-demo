pub use entities::*;
use proto_convert::derive::ProtoConvert;
use proto_convert::{ProtoConvert, ProtoConvertPrimitive};

pub mod entities {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "entities::Entity")]
pub struct EntityCommon {
    pub id: u32,
}
