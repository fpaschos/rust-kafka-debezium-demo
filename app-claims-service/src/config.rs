use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub log: claims_core::config::Log,
    pub db: Database,
    pub schema_registry: SchemaRegistry,
    pub server: Server,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaRegistry {
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Kafka {
    // pub schema_registry_url: Option<String>,
}
