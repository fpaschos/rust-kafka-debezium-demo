//! Run with
//!
//! ```not_rust
//! cargo run -p schema-registry-serialization
//! ```
//!
//! Note a running instance of schema registry is required with the available schemas registered.
mod record_producer;

use anyhow::Context;
use claims_schema::protos;
use claims_schema::protos::claimStatus::ClaimStatus::OPEN;
use protobuf::Message;
use tracing_subscriber::fmt::Subscriber;
use crate::record_producer::{get_producer, ProtoTopicMessage};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    let subscriber = Subscriber::builder()
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("setting default subscriber failed")?;


    let schema_registry_url = "http://localhost:58003";
    let brokers = "localhost:59092";
    let producer = get_producer(brokers, schema_registry_url);


    // Create protobuf entity
    let claim = protos::claim::Claim {
        id: 10,
        claim_no: "FOO".into(),
        status: OPEN.into(),
        ..Default::default()
    };

    // Encode to protobuf payload
    let payload = claim.write_to_bytes()?;
    let key = claim.id.to_string();

    let m = ProtoTopicMessage {
        topic: &"claims.test",
        payload: &payload,
        key: key.as_bytes(),
    };

    producer.send_message(m).await?;




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

    Ok(())
}
