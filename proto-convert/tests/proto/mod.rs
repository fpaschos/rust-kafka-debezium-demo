include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub mod protobuf {
    pub use super::entities::*;
    pub use super::timestamps::*;
}

pub mod prost {}
