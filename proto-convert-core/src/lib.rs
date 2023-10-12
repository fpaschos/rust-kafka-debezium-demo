use heck::{ToShoutySnakeCase, ToSnakeCase};
use syn::{Attribute, Meta};

mod attributes;
mod experimental;
pub mod impl_proto_convert;
mod proto_convert_enum;
mod proto_convert_struct;

#[cfg(test)]
mod tests;

const CONVERT_ATTRIBUTE: &str = "proto_convert";
const SNAKE_CASE_ATTRIBUTE_VALUE: &str = "snake_case";

const SCREAMING_SNAKE_CASE_ATTRIBUTE_VALUE: &str = "STREAMING_SNAKE_CASE";

pub(crate) fn find_proto_convert_meta(attrs: &[Attribute]) -> Option<&Meta> {
    attrs
        .iter()
        .find(|a| a.path().is_ident(CONVERT_ATTRIBUTE))
        .map(|a| &a.meta)
}

pub(crate) fn rename_item(item: &str, to_case: &str) -> darling::Result<String> {
    match to_case {
        SNAKE_CASE_ATTRIBUTE_VALUE => Ok(item.to_string().to_snake_case()),
        SCREAMING_SNAKE_CASE_ATTRIBUTE_VALUE => Ok(item.to_string().to_shouty_snake_case()),

        _ => Err(darling::Error::unknown_value(&format!(
            "Unknown rename case attribute = `{}` ",
            to_case
        ))),
    }
}
