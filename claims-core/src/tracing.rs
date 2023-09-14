use crate::config;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

// TODO configure app log level (using config) and modules log level
pub fn setup_tracing(_config: &config::Log) -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}