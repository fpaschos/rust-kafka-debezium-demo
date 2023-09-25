use std::time::Duration;

use anyhow::Context;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub trait ProtoMessage {
    fn key(&self) -> Vec<u8>;

    fn payload(&self) -> anyhow::Result<Vec<u8>>;

    fn full_name(&self) -> &'static str;

    fn key_full_name(&self) -> Option<&'static str>;
}

#[derive(Debug)]
pub struct ProtoEncodedMessage {
    key: Vec<u8>,
    payload: Vec<u8>,
}

impl ProtoEncodedMessage {
    #[inline]
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    #[inline]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Extensions trait that with more ergonomic api for encoding [`ProtoMessage`]s
/// See also implementation of [`EasyProtoRawEncoder`]
#[async_trait]
pub trait ProtoEncoder {
    async fn encode<M: ProtoMessage + Send + Sync>(
        &self,
        m: M,
        payload_strategy: SubjectNameStrategy,
        key_strategy: Option<SubjectNameStrategy>,
    ) -> anyhow::Result<ProtoEncodedMessage>;

    async fn encode_topic_name<M: ProtoMessage + Send + Sync>(
        &self,
        topic: &str,
        m: M,
    ) -> anyhow::Result<ProtoEncodedMessage> {
        let strat = SubjectNameStrategy::TopicNameStrategy(topic.into(), false);
        let key_strat = SubjectNameStrategy::TopicNameStrategy(topic.into(), true);
        self.encode(m, strat, Some(key_strat)).await
    }

    async fn encode_topic_name_raw_key<M: ProtoMessage + Send + Sync>(
        &self,
        topic: &str,
        m: M,
    ) -> anyhow::Result<ProtoEncodedMessage> {
        let strat = SubjectNameStrategy::TopicNameStrategy(topic.into(), false);
        self.encode(m, strat, None).await
    }
}

/// Implementation of [`ProtoEncoder`] extensions for [`EasyProtoRawEncoder`]
#[async_trait]
impl ProtoEncoder for EasyProtoRawEncoder {
    async fn encode<M: ProtoMessage + Send + Sync>(
        &self,
        m: M,
        payload_strategy: SubjectNameStrategy,
        key_strategy: Option<SubjectNameStrategy>,
    ) -> anyhow::Result<ProtoEncodedMessage> {
        let payload = m.payload()?;
        let full_name = m.full_name().to_owned();

        let encoded_value = self
            .encode(&payload, &full_name, payload_strategy)
            .await
            .context("Failed to proto encode payload for subject")?;

        let key = {
            // If there is available key_strategy try to encode key
            if let Some(key_strat) = key_strategy {
                if let Some(key_full_name) = m.key_full_name() {
                    let key_full_name = key_full_name.to_owned();
                    self.encode(&m.key(), &key_full_name, key_strat)
                        .await
                        .context("Failed to proto encode key for subject")?
                } else {
                    self.encode_single_message(&m.key(), key_strat)
                        .await
                        .context("Failed to proto encode single key for subject")?
                }
            } else {
                // In case of no strategy pass the key as raw bytes

                m.key()
            }
        };

        let encoded_key = key;
        Ok(ProtoEncodedMessage {
            key: encoded_key,
            payload: encoded_value,
        })
    }
}

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
