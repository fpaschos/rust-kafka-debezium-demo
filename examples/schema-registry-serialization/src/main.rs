//! Run with
//!
//! ```not_rust
//! cargo run -p schema-registry-serialization
//! ```
//!
//! Note a running instance of schema registry is required with the available schemas registered.
mod record_producer;

use crate::record_producer::{get_producer, ProtoMessage};
use anyhow::Context;
use claims_schema::protos;
use claims_schema::protos::claimStatus::ClaimStatus::OPEN;
use const_format::concatcp;
use protobuf::Message;
use tracing_subscriber::fmt::Subscriber;

const CLAIMS_SCHEMA: &'static str = "claims.schema.";
pub trait SchemaName {
    fn full_name(&self) -> &'static str;
}

impl SchemaName for protos::claim::Claim {
    fn full_name(&self) -> &'static str {
        &concatcp!(CLAIMS_SCHEMA, protos::claim::Claim::NAME)
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
}

// impl ProtoMessage for protos::claim::Claim {
//     #[inline]
//     fn key(&self) -> Vec<u8> {
//         self.id.to_be_bytes().into()
//     }
//
//     fn payload(&self) -> anyhow::Result<Vec<u8>> {
//         let payload = self.write_to_bytes()?;
//         Ok(payload)
//     }
//
//     fn full_name(&self) -> &'static str {
//         "claims.schema.Claim"
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    let subscriber = Subscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

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

    producer
        .send_proto(
            "claims.test",
            MessageKeyPair(&claim, &claim.id.to_be_bytes()),
        )
        .await?;
    producer
        .send_proto(
            "claims.test",
            MessageKeyPair(&claim, &claim.id.to_be_bytes()),
        )
        .await?;
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

    Ok(())
}
