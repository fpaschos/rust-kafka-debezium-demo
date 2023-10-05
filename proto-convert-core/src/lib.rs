use syn::{Attribute, Meta};

pub mod impl_proto_convert;

const CONVERT_ATTRIBUTE: &str = "proto_convert";

pub(crate) fn find_proto_convert_meta(attrs: &[Attribute]) -> Option<&Meta> {
    attrs
        .iter()
        .find(|a| a.path().is_ident(CONVERT_ATTRIBUTE))
        .map(|a| &a.meta)
}
