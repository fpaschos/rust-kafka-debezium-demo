use config::Config;
use serde::{Deserialize, Serialize};

pub fn load(path: &str) -> anyhow::Result<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()?;

    let config:AppConfig = config.try_deserialize()?;
    Ok(config)
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig{
    pub db: Database,
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