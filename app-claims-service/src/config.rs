use claims_core::config::{Database, SchemaRegistry, Server};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: claims_core::config::Log,
    pub db: Database,
    pub schema_registry: SchemaRegistry,
    pub server: Server,
}
