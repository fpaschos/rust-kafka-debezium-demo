use config::Config;
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: LogLevel,
}

#[derive(Debug, Deserialize)]
pub struct LogLevel {
    pub root: Option<String>,
    pub directives: Vec<LogDirective>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LogDirective {
    pub namespace: String,
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct SchemaRegistry {
    pub url: String,
}

#[derive(Debug, Deserialize)]
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
