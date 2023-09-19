//! Run with
//!
//! ```not_rust
//! cargo run -p schema-registry-serialization
//! ```
//!
//! Note a running instance of schema registry is required with the available schemas registered.
use anyhow::Context;
use claims_schema::protos;
use claims_schema::protos::claimStatus::ClaimStatus::OPEN;
use protobuf::Message;
use schema_registry_converter::async_impl::{
        easy_proto_raw::EasyProtoRawEncoder,
        schema_registry::SrSettings
    };
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawDecoder;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy::RecordNameStrategy;
use tracing_subscriber::fmt::Subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger with a specific log level (e.g., Level::INFO)
    let subscriber = Subscriber::builder()
        .finish();

    // Use the logger
    tracing::subscriber::set_global_default(subscriber).context("setting default subscriber failed")?;


    // Create protobuf entity
    let claim = protos::claim::Claim {
        id: 10,
        claim_no: "FOO".into(),
        status: OPEN.into(),
        age: 10,
        ..Default::default()
    };

    // Encode to protobuf payload
    let payload = claim.write_to_bytes()?;

    // Setup registry topic naming strategy
    let value_strategy = RecordNameStrategy("Claim".into());

    // Create schema registry proto encoder
    let settings = SrSettings::new("http://localhost:58003".into());
    let proto_encoder = EasyProtoRawEncoder::new(settings.clone());


    let encoded_value = proto_encoder.encode_single_message(&payload, value_strategy).await?;

    // Define schema registry proto decoder
    let proto_decoder = EasyProtoRawDecoder::new(settings);
    let decoded_value = proto_decoder.decode(Some(&encoded_value)).await?.unwrap();
    let decoded_claim = protos::claim::Claim::parse_from_bytes(&decoded_value.bytes)?;
    assert_eq!(decoded_claim, claim);

    tracing::info!("Decoded claim {:?}", decoded_claim);

    Ok(())
}
