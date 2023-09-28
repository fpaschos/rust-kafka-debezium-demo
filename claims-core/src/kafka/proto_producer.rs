use crate::proto_encode::encoder::ProtoEncoder;
use crate::proto_encode::message::ProtoMessage;
use anyhow::Context;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use std::time::Duration;

/// Producer capable of encoding protobuf messages using `schema registry`
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

    /// Sends a protobuf message to a topic using `SubjectTopicName` encoding for the payload
    /// and optionally for the key of the message.
    ///
    /// If `encode_key` false is selected the key is send as raw binary bytes.
    pub async fn send_topic_name<M: ProtoMessage + Send + Sync>(
        &self,
        topic: &str,
        m: M,
        encode_key: bool,
    ) -> anyhow::Result<()> {
        let encoded_kv = {
            if encode_key {
                self.proto_encoder.encode_topic_name(topic, m).await?
            } else {
                self.proto_encoder
                    .encode_topic_name_raw_key(topic, m)
                    .await?
            }
        };

        // Construct and send record from the encoded message
        let record = FutureRecord::to(topic)
            .key(encoded_kv.key())
            .payload(encoded_kv.payload());
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
