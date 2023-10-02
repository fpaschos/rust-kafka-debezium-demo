use config::Config;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Log {
    pub level: String,
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
    pub brokers: String,
}

pub fn load<C: DeserializeOwned>(path: &str) -> anyhow::Result<C> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()?;

    let config: C = config.try_deserialize()?;
    Ok(config)
}
