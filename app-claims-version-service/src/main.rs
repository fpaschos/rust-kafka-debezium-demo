use anyhow::Context;
use claims_core::tracing::setup_tracing;
use crate::config::AppConfig;

mod common;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig = claims_core::config::load(&"./config/application.yml").context("Unable to load config")?;
    setup_tracing(&config.log)?;
    tracing::info!("Test");
    tracing::warn!("Test");
    tracing::debug!("Test");
    tracing::trace!("Test");
    Ok(())
}


pub async fn consume() {}
