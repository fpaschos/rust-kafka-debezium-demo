use std::time::Duration;
use anyhow::Context;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub struct ProtoTopicMessage<'m> {
    pub topic: &'m str,
    pub key: &'m [u8],
    pub payload: &'m [u8],
}


pub struct RecordProducer {
    producer: FutureProducer,
    proto_encoder: EasyProtoRawEncoder,
}


impl RecordProducer {
    pub async fn send_message(&self, m: ProtoTopicMessage<'_>) -> anyhow::Result<()> {
        let value_strategy =
            SubjectNameStrategy::TopicNameStrategy(String::from(m.topic), false);
            // SubjectNameStrategy::RecordNameStrategy("Claim".into());

        let payload =
            self.proto_encoder
                .encode_single_message(m.payload, value_strategy)
                .await
                .context("Failed to proto encode message for topic")?;

        let record = FutureRecord::to(m.topic)
            .key(m.key)
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
        proto_encoder,
    }
}