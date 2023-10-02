use anyhow::Context;
use tokio::task::JoinHandle;

use claims_core::kafka::proto_consumer;
use claims_core::tracing::setup_tracing;
use claims_model::model::proto::claim::Claim;

use crate::config::AppConfig;

mod common;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig =
        claims_core::config::load("./config/application.yml").context("Unable to load config")?;
    setup_tracing(&config.log)?;

    tokio::select! {
        res = spawn_claims_consumer(&config) => {
            if let Ok(Err(error)) = res {
                tracing::error!("{}",error);
            }
        },
        _ = claims_core::shutdown::shutdown_signal() => {},
    }

    Ok(())
}

pub struct ClaimsHandler;

impl ClaimsHandler {
    pub async fn handle(&self, claim: Claim) -> anyhow::Result<()> {
        tracing::info!("Consumed claim: {}", claim);
        // anyhow::bail!("test");
        Ok(())
    }
}

pub fn spawn_claims_consumer(config: &AppConfig) -> JoinHandle<anyhow::Result<()>> {
    let consumer = proto_consumer::get_consumer(
        config.kafka.brokers.as_ref(),
        config.schema_registry.url.as_ref(),
        "app-claim-version",
        "claimsdb.claim.events",
    );

    let handler = ClaimsHandler;

    // Spawn a task to consume messages
    let consumer = tokio::spawn(async move {
        consumer
            .consume(|c| async { handler.handle(c).await })
            .await
    });
    consumer
}
