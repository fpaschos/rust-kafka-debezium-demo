//! Run with
//!
//! ```not_rust
//! cargo run -p schema-registry-serialization
//! ```
//!
//! Note a running instance of schema registry is required with the available schemas registered.
use anyhow::Context;
use protobuf::Message;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tracing_subscriber::fmt::Subscriber;

use claims_schema::protos::claim::Claim;
use claims_schema::protos::claimStatus::ClaimStatus::OPEN;
use proto_producer::ProtoEncoder;

use crate::proto_consumer::get_consumer;
use crate::proto_producer::{get_producer, ProtoMessage};

mod proto_consumer;
mod proto_producer;

// TODO keep only main here and move all the finalized code to claims-core lib project
const CLAIMS_SCHEMA: &str = "claims.schema.";
pub trait SchemaName {
    fn full_name(&self) -> &'static str;
}

pub trait KeySchemaName {
    fn key_full_name(&self) -> &'static str;
}

macro_rules! schema_name {
    ( $schema_literal:expr, $struct_name:ident) => {
        impl SchemaName for $struct_name {
            fn full_name(&self) -> &'static str {
                const_format::concatcp!($schema_literal, stringify!($struct_name))
            }
        }
    };
}

schema_name!(CLAIMS_SCHEMA, Claim);

struct MessageKeyPair<'m, M>(&'m M, &'m [u8]);

impl<'m, M: SchemaName + Message> ProtoMessage for MessageKeyPair<'m, M> {
    #[inline]
    fn key(&self) -> Vec<u8> {
        self.1.into()
    }

    #[inline]
    fn payload(&self) -> anyhow::Result<Vec<u8>> {
        let payload = self.0.write_to_bytes()?;
        Ok(payload)
    }

    #[inline]
    fn full_name(&self) -> &'static str {
        self.0.full_name()
    }

    #[inline]
    fn key_full_name(&self) -> Option<&'static str> {
        None
    }
}

// Example message handler
#[derive(Clone, Default)]
struct CountingMessageHandler {
    counter: Arc<AtomicU32>,
}

impl CountingMessageHandler {
    #[allow(dead_code)]
    pub async fn handle_message(&self, claim: Claim) {
        let c = self.counter.fetch_add(1, Ordering::SeqCst);
        tracing::info!("Counter = {} Consumed {}", c, claim);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    let subscriber = Subscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let schema_registry_url = "http://localhost:58003";
    let brokers = "localhost:59092";
    let producer = get_producer(brokers, schema_registry_url);
    let consumer = get_consumer(
        brokers,
        schema_registry_url,
        "example_claim_consumer",
        "claims.test",
    );

    let handler = CountingMessageHandler::default();

    // Spawn a task to consume messages
    let consumer = tokio::spawn(async move {
        consumer
            .consume(move |c| {
                let handler = handler.clone();
                async move { handler.handle_message(c).await }
            })
            .await
    });

    // Start to send proto messages on this task
    // Create protobuf entity
    let claim = Claim {
        id: 10,
        claim_no: "Fotis Paschos".into(),
        status: OPEN.into(),
        ..Default::default()
    };

    let settings = SrSettings::new(schema_registry_url.into());
    let proto_encoder = EasyProtoRawEncoder::new(settings);

    // Example of using ProtoEncoder
    let v = proto_encoder
        .encode_topic_name_raw_key(
            "claims.test",
            MessageKeyPair(&claim, claim.id.to_string().as_bytes()),
        )
        .await?;
    tracing::info!("{:?} {}", v, String::from_utf8(v.payload().to_vec())?);

    // Example of sending multiple times the same message
    for _i in 0..2 {
        producer
            .send_topic_name(
                "claims.test",
                MessageKeyPair(&claim, claim.id.to_string().as_bytes()),
                false,
            )
            .await?;
        tracing::info!("Claim message send successfully")
    }

    // Wait for consumer to terminate
    if let Ok(Err(err)) = consumer.await {
        tracing::error!("Consumer terminated with error: {}", err);
    }
    tracing::info!("Main task terminated");

    Ok(())
}
