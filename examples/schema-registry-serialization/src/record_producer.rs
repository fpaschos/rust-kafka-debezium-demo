use std::sync::Arc;
use std::time::Duration;
use anyhow::Context;
use protobuf::{Message, MessageDyn};
use rdkafka::ClientConfig;
use rdkafka::message::ToBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use claims_schema::protos::claim::Claim;


trait ProtoMessage {
    fn key_payload(&self) -> anyhow::Result<(Vec<u8>, Vec<u8>)>;
}

impl ProtoMessage for Claim {
    fn key_payload(&self) ->  anyhow::Result<(Vec<u8>, Vec<u8>)>  {
        let payload = self.write_to_bytes()?;
        let id = &self.id;
        Ok((id.to_be_bytes().into(), payload))
    }
}




#[derive(Clone)]
pub struct RecordProducer {
    producer: FutureProducer,
    proto_encoder: Arc<EasyProtoRawEncoder>,
}



impl RecordProducer {
    pub async fn send_proto(&self, topic: &str, m: &impl ProtoMessage) -> anyhow::Result<()> {
        let (key, payload) = m.key_payload()?;
        self.send_raw(topic, &key.to_bytes(), &payload ).await?;
        Ok(())
    }

    pub async fn send(&self, topic: &str, key: impl ToBytes, m: &impl Message) -> anyhow::Result<()> {
        let payload = m.write_to_bytes()?;
        self.send_raw(topic, &key.to_bytes(), &payload ).await?;
        Ok(())
    }

    async fn send_raw(&self, topic: &str, key: &[u8], payload: &[u8]) -> anyhow::Result<()> {
        let value_strategy =
            SubjectNameStrategy::TopicNameStrategy(topic.into(), false);
        // let key = key.to_bytes();
        let payload =
            self.proto_encoder
                .encode_single_message(payload, value_strategy)
                .await
                .context("Failed to proto encode message for topic")?;

        let record = FutureRecord::to(topic)
            .key(key)
            .payload(&payload);
        self.producer.send(record, Duration::from_secs(0)).await.map_err(|(e, _)| e).context("Failed to send kafka message")?;
        Ok(())
    }
}

pub fn get_producer<S: AsRef<str>>(brokers: S, schema_registry_url: S) -> RecordProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers.as_ref())
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "60000")
        .set("queue.buffering.max.messages", "10")
        .create()
        .expect("Producer creation error"); // No error handling in examples


    let settings = SrSettings::new(schema_registry_url.as_ref().into());
    let proto_encoder = EasyProtoRawEncoder::new(settings);
    RecordProducer {
        producer,
        proto_encoder: Arc::new(proto_encoder),
    }
}