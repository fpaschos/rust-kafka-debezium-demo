/// Trait that represents a message encoded via protobuf
pub trait ProtoMessage {
    fn key(&self) -> Vec<u8>;

    fn payload(&self) -> anyhow::Result<Vec<u8>>;

    fn full_name(&self) -> &'static str;

    fn key_full_name(&self) -> Option<&'static str>;
}

/// Trait that provides the full schema name of an entity
pub trait SchemaName {
    fn full_name(&self) -> &'static str;
}

#[macro_export]
/// Helper macro to implement schema name for an entity
macro_rules! schema_name_impl {
    ($schema_literal:expr, $struct_name:ident) => {
        impl SchemaName for $struct_name {
            fn full_name(&self) -> &'static str {
                const_format::concatcp!($schema_literal, stringify!($struct_name))
            }
        }
    };
}

/// Trait that provides the key full schema name of an entity
pub trait KeySchemaName {
    fn key_full_name(&self) -> &'static str;
}

/// Helper intermediate struct for providing ergonomic api for working with `protobuf::Message`s
pub struct MessageKeyPair<'m, M>(pub &'m M, pub &'m [u8]);

/// Implementation of [`ProtoMessage`] for [`MessageKeyPair`]
impl<'m, M: SchemaName + protobuf::Message> ProtoMessage for MessageKeyPair<'m, M> {
    #[inline]
    fn key(&self) -> Vec<u8> {
        self.1.into()
    }

    #[inline]
    fn payload(&self) -> anyhow::Result<Vec<u8>> {
        let payload = self.0.write_to_bytes()?;
        Ok(payload)
    }

    #[inline]
    fn full_name(&self) -> &'static str {
        self.0.full_name()
    }

    #[inline]
    fn key_full_name(&self) -> Option<&'static str> {
        None
    }
}
