use proc_macro2::Ident;
use std::default::Default;
use syn::fold::Fold;
use syn::{parse_quote, Path, PathArguments, Type, TypePath};

#[derive(Debug, Clone)]
enum NestedType {
    Named {
        name: Ident,
        args: Vec<Box<NestedType>>,
    },
    Unnamed {
        args: Vec<Box<NestedType>>,
    },
}

impl NestedType {
    fn new(ident: Option<&Ident>) -> Self {
        if let Some(ident) = ident {
            Self::Named {
                name: ident.clone(),
                args: Default::default(),
            }
        } else {
            Self::Unnamed {
                args: Default::default(),
            }
        }
    }
}

impl NestedType {
    fn name(&self) -> Option<String> {
        match self {
            NestedType::Named { name, .. } => Some(name.to_string()),
            NestedType::Unnamed { .. } => None,
        }
    }

    #[inline]
    fn args(&self) -> &Vec<Box<NestedType>> {
        match self {
            NestedType::Named { args, .. } => args,
            NestedType::Unnamed { args, .. } => args,
        }
    }

    #[inline]
    fn args_mut(&mut self) -> &mut Vec<Box<NestedType>> {
        match self {
            NestedType::Named { args, .. } => args,
            NestedType::Unnamed { args, .. } => args,
        }
    }

    #[inline]
    fn nest(&mut self, other: NestedType) {
        self.args_mut().push(Box::new(other));
    }
}

// TODO mark as #[cfg(test)] used only for test purposes
impl ToString for NestedType {
    fn to_string(&self) -> String {
        let mut res = String::new();

        let name = self.name();
        if let Some(name) = &name {
            res.push_str(name);
        }

        if self.args().is_empty() {
            return res;
        }
        if name.is_some() {
            res.push('<');
        } else {
            res.push('(')
        }

        let args: Vec<_> = self.args().iter().map(|arg| arg.to_string()).collect();
        res.push_str(&args.join(","));
        if name.is_some() {
            res.push('>');
        } else {
            res.push(')')
        }

        res
    }
}

#[derive(Debug, Default, Clone)]
struct TypeScanner {
    stack: Vec<NestedType>,
}

impl TypeScanner {
    fn scan(&mut self, p: Path) -> NestedType {
        self.fold_path(p);
        debug_assert_eq!(
            self.stack.len(),
            1,
            "Invalid TypeScanner::scan result stack contains more than one root elements"
        );
        self.stack.pop().unwrap()
    }

    fn stack_un_nest_top(&mut self) -> bool {
        debug_assert!(
            !self.stack.is_empty(),
            "Invalid TypeScanner::stack_nest_top stack is empty"
        );

        let top = self.stack.last_mut().unwrap();
        if let Some(nested_last) = top.args_mut().pop() {
            self.stack.push(*nested_last);
            true
        } else {
            false
        }
    }

    fn stack_nest_top(&mut self) {
        debug_assert!(
            self.stack.len() > 1,
            "Invalid TypeScanner::stack_nest_top stack size < 2"
        );
        let top = self.stack.pop().unwrap();
        let pre_top = self.stack.last_mut().unwrap();
        pre_top.nest(top);
    }
}

impl Fold for TypeScanner {
    // fn fold_type_path(&mut self, i: TypePath) -> TypePath {
    //     todo!()
    // }
    //
    // fn fold_type(&mut self, i: Type) -> Type {
    //     todo!()
    // }
    fn fold_path(&mut self, p: Path) -> Path {
        let last = p.segments.last().map(|s| &s.ident);
        let ty = NestedType::new(last);

        if self.stack.is_empty() {
            self.stack.push(ty);
        } else {
            if let Some(last) = self.stack.last_mut() {
                last.nest(ty);
            }
        }

        let inner = syn::fold::fold_path(self, p);
        inner
    }

    fn fold_path_arguments(&mut self, i: PathArguments) -> PathArguments {
        let un_nested = self.stack_un_nest_top();
        let inner = syn::fold::fold_path_arguments(self, i);
        if un_nested {
            self.stack_nest_top();
        }
        inner
    }
}
#[test]
fn test_fold_type() {
    let fragment: Path = parse_quote! {
      HashMap<u8,u8>
    };

    let mut scanner = TypeScanner::default();
    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<u8>
    };

    let mut scanner = TypeScanner::default();
    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<Vec<u8>>
    };

    let mut scanner = TypeScanner::default();
    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      std::collections::HashMap<u8,Vec<u8>>
    };

    let mut scanner = TypeScanner::default();
    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      Triple<u8,u8, Option<Vec<u8>>>
    };

    let mut scanner = TypeScanner::default();
    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Triple<u8,u8,Option<Vec<u8>>>".to_string());
}

// #[test]
// fn test_weird_types() {
//     let fragment: Path = parse_quote! {
//       u8
//     };
//
//     let mut scanner = TypeScanner::default();
//     let res = scanner.scan(fragment);
//     assert_eq!(res.to_string(), "u8".to_string());
//
//     let fragment: Path = parse_quote! {
//       Mutex<u8,u8, Option<Vec<(u8,u8)>>>
//     };
//
//     let mut scanner = TypeScanner::default();
//     let res = scanner.scan(fragment);
//     assert_eq!(
//         res.to_string(),
//         "Mutex<u8,u8,Option<Vec<(u8,u8)>>>".to_string()
//     );
// }
