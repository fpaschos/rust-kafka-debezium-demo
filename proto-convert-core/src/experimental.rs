use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Attribute, Data, DataStruct, DeriveInput, GenericArgument, Path, PathArguments, Type};

use crate::find_proto_convert_meta;

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
    Bytes,
}

fn maybe_known_primitive_type(ident: &Ident) -> Option<PrimitiveTy> {
    match ident {
        _ if ident == "u32" => Some(PrimitiveTy::U32),
        _ if ident == "i32" => Some(PrimitiveTy::I32),
        _ if ident == "f32" => Some(PrimitiveTy::F32),
        _ if ident == "f64" => Some(PrimitiveTy::F64),
        _ if ident == "u64" => Some(PrimitiveTy::U64),
        _ if ident == "i64" => Some(PrimitiveTy::I64),
        _ if ident == "bool" => Some(PrimitiveTy::Bool),
        _ if ident == "String" => Some(PrimitiveTy::String),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Ty {
    Primitive { ty: PrimitiveTy, optional: bool },
    Enumeration { ty: Path, optional: bool },
    Other { ty: Path, optional: bool },
}

impl Ty {
    pub(crate) fn primitive(ty: PrimitiveTy, optional: bool) -> Self {
        Self::Primitive { ty, optional }
    }

    pub(crate) fn enumerations(ty: Path, optional: bool) -> Self {
        Self::Enumeration { ty, optional }
    }

    pub(crate) fn other(ty: Path, optional: bool) -> Self {
        Self::Other { ty, optional }
    }

    // TODO handle enumeration case via attrs
    fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        match &field.ty {
            Type::Path(syn::TypePath { path, .. }) => {
                let last_segment = path.segments.last().unwrap(); // TODO what if there is no last segment
                return match &last_segment.arguments {
                    PathArguments::None => {
                        // Check only for primitive types or any other type
                        if let Some(ty) = maybe_known_primitive_type(&last_segment.ident) {
                            Ok(Ty::primitive(ty, false))
                        } else {
                            Ok(Ty::other(path.clone(), false))
                        }
                    }
                    PathArguments::AngleBracketed(args) => {
                        if args.args.iter().count() > 1 {
                            // Types with more than one generic argument are classified as other
                            Ok(Ty::other(path.clone(), false))
                        } else {
                            if last_segment.ident == "Option" {
                                // Type is option check the inner type
                                let gen = args.args.iter().next().unwrap();

                                if let GenericArgument::Type(Type::Path(t)) = gen {
                                    let last_segment = t.path.segments.last().unwrap(); // TODO what if there is no last segment
                                                                                        // Check only for primitive types or any other type
                                    if let Some(ty) =
                                        maybe_known_primitive_type(&last_segment.ident)
                                    {
                                        Ok(Ty::primitive(ty, true))
                                    } else {
                                        Ok(Ty::other(path.clone(), true))
                                    }
                                } else {
                                    Err(
                                        darling::Error::unexpected_type(
                                            "Macro only supports path argument types",
                                        ), // .with_span(&field.span()), TODO test this
                                    )
                                }
                            } else {
                                // The type is not Option return it as other non optional
                                Ok(Ty::other(path.clone(), false))
                            }
                        }
                    }
                    PathArguments::Parenthesized(_) => {
                        Err(
                            darling::Error::unexpected_type(
                                "Macro does not support parenthesized type arguments",
                            ), // .with_span(&field.span()), TODO test this
                        )
                    }
                };
            }
            _ => Err(
                darling::Error::unexpected_type("Macro supports only primitive path types"), // .with_span(&field.span()), TODO test this
            ),
        }
    }

    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        match self {
            Ty::Primitive { optional, .. } => *optional,
            Ty::Enumeration { optional, .. } => *optional,
            Ty::Other { optional, .. } => *optional,
        }
    }
}

pub(crate) struct StructField {
    pub name: String,
    pub ty: Ty,
    pub attrs: Option<FieldAttrs>,
}

impl StructField {
    #[inline]
    pub(crate) fn is_optional(&self) -> bool {
        self.ty.is_optional()
    }
}

impl StructField {
    pub(crate) fn try_from_field(field: &syn::Field) -> darling::Result<Self> {
        let name = field.ident.as_ref().ok_or_else(|| {
            darling::Error::unsupported_shape("Macro supports only structs with named fields")
        })?;

        let ty = Ty::try_from_field(field)?;

        Ok(Self {
            name: name.to_string(),
            ty,
            attrs: None,
        })
    }
}

pub(crate) struct Struct {
    pub name: String,
    pub attrs: StructAttrs,
    pub fields: Vec<StructField>,
}

impl Struct {
    pub(crate) fn try_from_data(
        name: &Ident,
        data: &DataStruct,
        attrs: &[Attribute],
    ) -> darling::Result<Self> {
        let meta = find_proto_convert_meta(attrs).ok_or_else(|| {
            darling::Error::unsupported_shape("Missing required proto attribute `proto_convert`")
        })?;

        let attrs = StructAttrs::from_meta(meta)?;

        let fields = data
            .fields
            .iter()
            .map(StructField::try_from_field)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: name.clone().to_string(),
            fields,
            attrs,
        })
    }
}

/// Meta attributes for `struct` items
#[derive(Debug, darling::FromMeta)]
pub(crate) struct StructAttrs {
    pub source: Path,
    /// Optional renaming of the struct fields before mapping to the proto entity.
    pub rename_all: Option<String>,
}

#[derive(Debug)]
pub(crate) struct FieldAttrs {}

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
