use crate::types::{NestedType, TypeScanner};
use syn::Type;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum PrimitiveTy {
    F32,
    F64,
    U32,
    I32,
    U64,
    I64,
    Bool,
    String,
    VecBytes,
    // Special case for enumerations fall back to u32
    Enumeration,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Ty {
    Primitive { ty: PrimitiveTy, optional: bool },
    Other { optional: bool },
}

impl Ty {
    pub(crate) fn primitive(ty: PrimitiveTy, optional: bool) -> Self {
        Self::Primitive { ty, optional }
    }

    pub(crate) fn other(optional: bool) -> Self {
        Self::Other { optional }
    }

    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        match self {
            Ty::Primitive { optional, .. } => *optional,
            Ty::Other { optional, .. } => *optional,
        }
    }

    #[inline]
    pub(crate) fn is_primitive(&self) -> bool {
        matches!(self, Ty::Primitive { .. })
    }

    // TODO handle enumeration case via attrs
    pub(crate) fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        let mut scanner = TypeScanner::default();

        match &field.ty {
            Type::Path(syn::TypePath { path, .. }) => {
                let nested_type = scanner.scan(path.clone());
                Ok(Ty::from(&nested_type))
            }
            _ => Err(
                darling::Error::unexpected_type("Macro supports only path types"), // .with_span(&field.span()), TODO test this
            ),
        }
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
