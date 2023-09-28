use crate::db::entities::ClaimOutboxEventDb;
use crate::db::events::send_event;
use crate::db::PostgresTx;
use claims_core::proto_encode::encoder::ProtoEncoder;
use claims_core::proto_encode::message::MessageKeyPair;
use claims_model::model::{proto, Claim};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use std::sync::Arc;

#[derive(Clone)]
pub struct EventService {
    proto_encoder: Arc<EasyProtoRawEncoder>,
}

impl EventService {
    pub fn new<S: AsRef<str>>(schema_registry_url: S) -> Self {
        let settings = SrSettings::new(schema_registry_url.as_ref().into());
        let proto_encoder = EasyProtoRawEncoder::new(settings);
        Self {
            proto_encoder: Arc::new(proto_encoder),
        }
    }
    pub async fn send_claim(&self, tx: &mut PostgresTx<'_>, claim: &Claim) -> anyhow::Result<()> {
        // Create the protobuf message from Claim

        let proto: proto::claim::Claim = claim.clone().into();

        // Encode the protobuf
        let encoded = self
            .proto_encoder
            .encode_topic_name_raw_key(
                "claimsdb.claim.events",
                MessageKeyPair(&proto, proto.id.to_string().as_bytes()),
            )
            .await?;

        // Send the message via the outbox table
        let event = ClaimOutboxEventDb {
            aggregatetype: "claim".into(),
            aggregateid: claim.id.to_string(),
            r#type: "update".into(),
            payload: encoded.payload().into(),
            ..Default::default()
        };
        let _ = send_event(tx, event).await?;
        Ok(())
    }
}
