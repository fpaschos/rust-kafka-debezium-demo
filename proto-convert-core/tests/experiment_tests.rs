#![cfg(test)]

use quote::quote;
use syn::Fields::Named;
use syn::{ItemStruct, Type};

#[test]
fn parse2_extract_types_test() {
    let fragment = quote! {
        #[derive(Debug, PartialEq)]
        struct Entity {
            pub id: u32,
            pub opt_id: std::option::Option<u32>,
            pub field: Whatever,
            pub opt_field: Option<Whatever>,
        }
    };

    let parsed = syn::parse2::<ItemStruct>(fragment.into()).unwrap();

    let types: Vec<_> = if let Named(fields) = parsed.fields {
        fields
            .named
            .iter()
            .filter_map(|f| {
                match &f.ty {
                    Type::Path(path) => {
                        let ident = path.path.segments.last().map(|s| &s.ident);
                        Some(ident)
                    }
                    _ => None, // Do nothing
                }
            })
            .map(|v| v.unwrap())
            .cloned()
            .collect()
    } else {
        vec![]
    };

    assert_eq!(
        types.iter().map(|i| i.to_string()).collect::<Vec<_>>(),
        vec!["u32", "Option", "Whatever", "Option"] // Expected
    );
}
