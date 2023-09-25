//! Run with
//!
//! ```not_rust
//! cargo run -p schema-registry-serialization
//! ```
//!
//! Note a running instance of schema registry is required with the available schemas registered.
use anyhow::Context;
use const_format::concatcp;
use protobuf::Message;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use tracing_subscriber::fmt::Subscriber;

use claims_schema::protos;
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

impl SchemaName for Claim {
    fn full_name(&self) -> &'static str {
        concatcp!(CLAIMS_SCHEMA, Claim::NAME)
    }
}

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
struct MessageHandler {
    name: String,
}

impl MessageHandler {
    #[allow(dead_code)]
    pub fn handle_message(&self, claim: Claim) {
        tracing::info!("{} Consumed {}", self.name, claim);
    }

    #[allow(dead_code)]
    pub fn handle_message_mut(&mut self, claim: Claim) {
        self.name.push('+');
        tracing::info!("{} Consumed {}", self.name, claim);
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

    let mut handler = MessageHandler {
        name: "TEST_HANDLER".into(),
    };

    // Spawn a task to consume messages
    let consumer =
        tokio::spawn(async move { consumer.consume(|c| handler.handle_message_mut(c)).await });

    // Start to send proto messages on this task
    // Create protobuf entity
    let claim = protos::claim::Claim {
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

    // Example of sending multiple times the same messagex
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
    // producer.send_proto("claims.test", claim).await?;
    // Encode to protobuf payload
    // Encode to protobuf payload
    // let m = claim.for_topic("claims.test")?;
    // producer.send_message(m).await?;

    // Create schema registry proto encoder
    // let settings = SrSettings::new("http://localhost:58003".into());
    // let proto_encoder = EasyProtoRawEncoder::new(settings.clone());

    // let encoded_value = proto_encoder.encode_single_message(&payload, value_strategy).await?;
    //
    // // Define schema registry proto decoder
    // let proto_decoder = EasyProtoRawDecoder::new(settings);
    // let decoded_value = proto_decoder.decode(Some(&encoded_value)).await?.unwrap();
    // let decoded_claim = protos::claim::Claim::parse_from_bytes(&decoded_value.bytes)?;
    // assert_eq!(decoded_claim, claim);
    //
    // tracing::info!("Decoded claim {:?}", decoded_claim);

    // Wait for consumer to terminate
    if let Ok(Err(err)) = consumer.await {
        tracing::error!("Consumer terminated with error: {}", err);
    }
    tracing::info!("Main task terminated");

    Ok(())
}
