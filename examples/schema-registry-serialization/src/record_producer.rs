use std::time::Duration;

use anyhow::Context;
use protobuf::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub trait ProtoMessage {
    fn key(&self) -> Vec<u8>;
    fn payload(&self) -> anyhow::Result<Vec<u8>>;

    fn full_name(&self) -> &'static str;
}

pub struct ProtoProducer {
    producer: FutureProducer,
    proto_encoder: EasyProtoRawEncoder,
}

impl ProtoProducer {
    pub fn new(producer: FutureProducer, proto_encoder: EasyProtoRawEncoder) -> Self {
        Self {
            producer,
            proto_encoder,
        }
    }

    pub async fn send_proto(&self, topic: &str, m: impl ProtoMessage) -> anyhow::Result<()> {
        let key = m.key();
        let payload = m.payload()?;

        self.send_proto_raw(topic, &key, &payload, Some(m.full_name()))
            .await?;
        Ok(())
    }

    pub async fn send_proto_raw(
        &self,
        topic: &str,
        key: &[u8],
        payload: &[u8],
        proto_full_name: Option<&str>,
    ) -> anyhow::Result<()> {
        let payload_subject = SubjectNameStrategy::TopicNameStrategy(topic.into(), false);

        let key = { key };

        let payload = {
            if let Some(full_name) = proto_full_name {
                self.proto_encoder
                    .encode(payload, full_name, payload_subject)
                    .await
                    .context("Failed to proto encode message for topic")?
            } else {
                self.proto_encoder
                    .encode_single_message(payload, payload_subject)
                    .await
                    .context("Failed to proto encode message for topic")?
            }
        };

        let record = FutureRecord::to(topic).key(key).payload(&payload);
        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|(e, _)| e)
            .context("Failed to send kafka message")?;
        Ok(())
    }
}

pub fn get_producer<S: AsRef<str>>(brokers: S, schema_registry_url: S) -> ProtoProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers.as_ref())
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "60000")
        .set("queue.buffering.max.messages", "10")
        .create()
        .expect("Producer creation error"); // No error handling in examples

    let settings = SrSettings::new(schema_registry_url.as_ref().into());
    let proto_encoder = EasyProtoRawEncoder::new(settings);

    ProtoProducer::new(producer, proto_encoder)
}
