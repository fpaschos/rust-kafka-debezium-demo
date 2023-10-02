use claims_core::config::{Kafka, Log, SchemaRegistry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub log: Log,
    pub schema_registry: SchemaRegistry,
    pub kafka: Kafka,
}
