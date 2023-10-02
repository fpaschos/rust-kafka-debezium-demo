use claims_core::config::{Kafka, Log, SchemaRegistry};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: Log,
    pub schema_registry: SchemaRegistry,
    pub kafka: Kafka,
}
