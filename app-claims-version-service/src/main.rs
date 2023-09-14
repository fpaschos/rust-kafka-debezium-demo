use anyhow::Context;
use futures::{StreamExt, TryStreamExt};
use rdkafka::{ClientConfig, ClientContext, Statistics, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use claims_core::tracing::setup_tracing;
use crate::config::AppConfig;

mod common;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig = claims_core::config::load(&"./config/application.yml").context("Unable to load config")?;
    setup_tracing(&config.log)?;
    // tracing::info!("Test");
    // tracing::warn!("Test");
    // tracing::debug!("Test");
    // tracing::trace!("Test");

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
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
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
        .set("enable.auto.commit", "false")
        // .set("auto.commit.interval.ms", "5000")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(CustomContext)
        // .create()
        .context("Kafka: Consumer creation failed")?;

    consumer.subscribe(&["claims-ns.claims.claim"]).context("Kafka: Unable to subscribe to topic")?;

    tracing::info!("Starting event loop");
    let mut stream = consumer.stream();
    loop {
        let message = stream.next().await;
        match message {
            Some(Ok(message)) => println!(
                "Received message: {}",
                match message.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => "<invalid utf-8>",
                }
            ),
            Some(Err(e)) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }

            None => {
                eprintln!("Consumer unexpectedly returned no messages");
                break;
            }
        }
    }


    // let stream_consumer = consumer.stream().try_for_each(|msg: BorrowedMessage| {
    //     tracing::debug!("Message received {}", &msg.offset());
    //     async move {
    //         tracing::debug!("Message received {}", &msg.offset());
    //         // tracing::debug!("Message received {}", msg.offset());
    //         Ok(())
    //     }
    // });

    // stream_consumer.await
    //     .context("Kafka: Stream consumer failed")?;
    tracing::info!("Stream processing terminated");
    Ok(())
}
