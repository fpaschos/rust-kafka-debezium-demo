use anyhow::Context;
use futures::{StreamExt, TryStreamExt};
use rdkafka::{ClientConfig, ClientContext, Statistics, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, ConsumerContext, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use claims_core::tracing::setup_tracing;
use crate::config::AppConfig;

mod common;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig = claims_core::config::load("./config/application.yml")
        .context("Unable to load config")?;
    setup_tracing(&config.log)?;
    run_consumer().await?;

    Ok(())
}

struct CustomContext;

impl ClientContext for CustomContext {
    fn log(&self, level: RDKafkaLogLevel, fac: &str, log_message: &str) {
        println!("log: {} {}", fac, log_message);
    }

    fn stats(&self, statistics: Statistics) {
        println!("stats: {:?}", statistics)
    }

    fn error(&self, error: KafkaError, reason: &str) {
        println!("error: {} {}", error, reason)
    }
}


impl ConsumerContext for CustomContext {

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        tracing::debug!("Committing offsets: {:?}", result);
    }
}


type CustomConsumer = StreamConsumer<CustomContext>;

pub async fn run_consumer() -> anyhow::Result<()> {

    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: CustomConsumer = ClientConfig::new()
        .set("group.id", "app-claims-version-consumer")
        .set("bootstrap.servers", "localhost:59092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "5000")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(CustomContext)
        .context("Kafka: Consumer creation failed")?;

    consumer
        .subscribe(&["claims-ns.claims.claim", "claims-ns.claims.party"])
        .context("Kafka: Unable to subscribe to topic")?;

    tracing::info!("Starting event loop");

    let stream_consumer = consumer.stream().try_for_each(|msg: BorrowedMessage| {
        tracing::debug!("Message received offset {}", &msg.offset());
        tracing::debug!("Message payload {:?}", &msg.payload());
        async move {
            // tracing::debug!("Message received {}", &msg.offset());
            // tracing::debug!("Message received {}", msg.offset());
            Ok(())
        }
    });

    stream_consumer.await
        .context("Kafka: Stream consumer failed")?;
    tracing::info!("Stream processing terminated");
    Ok(())
}
