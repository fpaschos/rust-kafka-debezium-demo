use config::Config;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Log {
    pub level: String,
}


pub fn load<C: DeserializeOwned>(path: &str) -> anyhow::Result<C> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()?;

    let config: C = config.try_deserialize()?;
    Ok(config)
}