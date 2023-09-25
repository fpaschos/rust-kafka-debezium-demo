use anyhow::{anyhow, Context};
use protobuf::Message;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message as KafkaMessage};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawDecoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use std::future::Future;

// TODO context for auto commit?
pub struct ProtoConsumer {
    consumer: StreamConsumer,
    proto_decoder: EasyProtoRawDecoder,
    topic: String,
}

impl ProtoConsumer {
    pub fn new<S: AsRef<str>>(
        consumer: StreamConsumer,
        proto_decoder: EasyProtoRawDecoder,
        topic: S,
    ) -> Self {
        Self {
            consumer,
            proto_decoder,
            topic: topic.as_ref().into(),
        }
    }

    pub async fn consume<M, Fut>(&self, handler: impl Fn(M) -> Fut) -> anyhow::Result<()>
    where
        M: Message,
        Fut: Future<Output = ()> + Send,
    {
        self.consumer
            .subscribe(&[&self.topic])
            .context(format!("Can't subscribe to topic {}", self.topic))?;

        // TODO handle kafka error
        while let Ok(message) = self.consumer.recv().await {
            tracing::info!("Begin handling message {}", message.offset());

            let decoded_payload = self.proto_decoder.decode(message.payload()).await?;
            let decoded_payload = decoded_payload.ok_or(anyhow!("Unable to decode payload"))?;

            let parsed_payload = Message::parse_from_bytes(&decoded_payload.bytes)?;

            handler(parsed_payload).await;

            // Commit the offsets
            self.consumer.commit_message(&message, CommitMode::Async)?;
        }

        Ok(())
    }
}

pub fn get_consumer<S: AsRef<str>>(
    brokers: S,
    schema_registry_url: S,
    group_id: S,
    topic: S,
) -> ProtoConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id.as_ref())
        .set("bootstrap.servers", brokers.as_ref())
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest") // Default start from the start of the stream
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation error");

    let settings = SrSettings::new(schema_registry_url.as_ref().into());
    let proto_decoder = EasyProtoRawDecoder::new(settings);
    ProtoConsumer::new(consumer, proto_decoder, topic)
}
