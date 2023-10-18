pub use entities::*;

pub mod entities {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}
