use anyhow::Context;
use tokio::task::JoinHandle;

use crate::config::AppConfig;
use claims_core::kafka::proto_consumer;
use claims_core::tracing::init;
use claims_model::model::proto::ProtoMap;
use claims_model::model::{proto, Claim, Party};

mod common;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig =
        claims_core::config::load("./config/application.yml").context("Unable to load config")?;
    init(&config.log)?;

    tokio::select! {
        res = spawn_claims_consumer(&config) => {
            if let Ok(Err(error)) = res {
                tracing::error!("{}",error);
            }
        },
        res = spawn_parties_consumer(&config) => {
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
    pub async fn handle(&self, proto: proto::claim::Claim) -> anyhow::Result<()> {
        let claim: Claim = Claim::from_proto(proto)?;
        tracing::debug!("Processing claim: {:?}", claim);
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
    tokio::spawn(async move {
        consumer
            .consume(|c| async { handler.handle(c).await })
            .await
    })
}

pub struct PartiesHandler;

impl PartiesHandler {
    pub async fn handle(&self, proto: proto::party::Party) -> anyhow::Result<()> {
        let party = Party::from_proto(proto)?;
        tracing::debug!("Processing party: {:?}", party);
        Ok(())
    }
}

pub fn spawn_parties_consumer(config: &AppConfig) -> JoinHandle<anyhow::Result<()>> {
    let consumer = proto_consumer::get_consumer(
        config.kafka.brokers.as_ref(),
        config.schema_registry.url.as_ref(),
        "app-claim-version",
        "claimsdb.party.events",
    );

    let handler = PartiesHandler;

    // Spawn a task to consume messages
    tokio::spawn(async move {
        consumer
            .consume(|c| async { handler.handle(c).await })
            .await
    })
}
