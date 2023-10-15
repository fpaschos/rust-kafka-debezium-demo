use crate::experimental::{PrimitiveTy, Ty};
use proc_macro2::Ident;
use std::default::Default;
use syn::fold::Fold;
use syn::{parse_quote, Path, PathArguments};

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
    pub fn args(&self) -> &Vec<Box<NestedType>> {
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

    pub(crate) fn is_named<S: AsRef<str>>(&self, n: S) -> bool {
        match self {
            NestedType::Named { name, .. } => *name == n.as_ref(),
            NestedType::Unnamed { .. } => false,
        }
    }
}

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

impl From<&NestedType> for Ty {
    fn from(value: &NestedType) -> Self {
        // TODO how do I traverse NestedType efficiently???
        let value = value.to_string();
        match value.to_string() {
            _ if value == "bool" => Self::primitive(PrimitiveTy::Bool, false),
            _ if value == "String" => Self::primitive(PrimitiveTy::String, false),
            _ if value == "u32" => Self::primitive(PrimitiveTy::U32, false),
            _ if value == "i32" => Self::primitive(PrimitiveTy::I32, false),
            _ if value == "f32" => Self::primitive(PrimitiveTy::F32, false),
            _ if value == "f64" => Self::primitive(PrimitiveTy::F64, false),
            _ if value == "u64" => Self::primitive(PrimitiveTy::U64, false),
            _ if value == "i64" => Self::primitive(PrimitiveTy::I64, false),
            _ if value == "Vec<u8>" => Self::primitive(PrimitiveTy::VecBytes, false),
            _ if value == "Option<bool>" => Self::primitive(PrimitiveTy::Bool, true),
            _ if value == "Option<String>" => Self::primitive(PrimitiveTy::String, true),
            _ if value == "Option<u32>" => Self::primitive(PrimitiveTy::U32, true),
            _ if value == "Option<i32>" => Self::primitive(PrimitiveTy::I32, true),
            _ if value == "Option<f32>" => Self::primitive(PrimitiveTy::F32, true),
            _ if value == "Option<f64>" => Self::primitive(PrimitiveTy::F64, true),
            _ if value == "Option<u64>" => Self::primitive(PrimitiveTy::U64, true),
            _ if value == "Option<i64>" => Self::primitive(PrimitiveTy::I64, true),
            _ if value == "Option<Vec<u8>>" => Self::primitive(PrimitiveTy::VecBytes, true),
            _ if value.starts_with("Option<") => Self::other(true),
            _ => Self::other(false),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct TypeScanner {
    stack: Vec<NestedType>,
}

impl TypeScanner {
    fn scan(&mut self, p: Path) -> NestedType {
        self.stack.clear();
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
    // TODO try to fold more generic types
    // fn fold_type_path(&mut self, ty: TypePath) -> TypePath {
    //     match
    // }
    //
    // fn fold_type(&mut self, i: Type) -> Type {
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
    let mut scanner = TypeScanner::default();

    let fragment: Path = parse_quote! {
      HashMap<u8,u8>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<u8>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<u8>".to_string());

    let fragment: Path = parse_quote! {
      std::string::Option<Vec<u8>>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "Option<Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      std::collections::HashMap<u8,Vec<u8>>
    };

    let res = scanner.scan(fragment);
    assert_eq!(res.to_string(), "HashMap<u8,Vec<u8>>".to_string());

    let fragment: Path = parse_quote! {
      Triple<u8,u8, Option<Vec<u8>>>
    };

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
