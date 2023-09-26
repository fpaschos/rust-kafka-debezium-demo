use anyhow::Context;
use async_trait::async_trait;
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

use crate::proto_encode::message::ProtoMessage;

/// Helper struct that holds an encoded [`ProtoMessage`]
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
