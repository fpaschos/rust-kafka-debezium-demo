use crate::proto_struct::Struct;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

mod implement_struct_tests;
mod parse_struct_tests;
mod type_scanner_tests;

#[cfg(test)]
pub(crate) fn from_derive_input(input: &DeriveInput) -> darling::Result<Struct> {
    match &input.data {
        Data::Struct(data) => {
            let s = Struct::try_from_data(&input.ident, data, &input.attrs)?;
            Ok(s)
        }

        //     Ok(ProtoConvert::Struct(ProtoConvertStruct::from_derive_input(
        //     input.ident.clone(),
        //     input.attrs.as_ref(),
        //     data,
        // )?)),
        _ => Err(darling::Error::unsupported_shape(
            "Macro supports only `struct` and `enum` items",
        )),
    }
}

pub fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
