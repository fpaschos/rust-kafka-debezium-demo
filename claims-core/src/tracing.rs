use crate::config;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;

pub fn init(config: &config::Log) -> anyhow::Result<()> {
    init_log_and_tracing(|mut e| {
        // Configure root level
        if let Some(root_level) = &config.level.root {
            e = e.add_directive(root_level.parse().unwrap_or_default())
        }

        // Configure specific directives
        for directive in &config.level.directives {
            let directive_string = format!("{}={}", directive.namespace, directive.level);
            e = e.add_directive(directive_string.parse().unwrap_or_default());
        }
        e
    })
}

pub fn init_log_and_tracing<T>(env_filter_customizer: T) -> anyhow::Result<()>
where
    T: Fn(EnvFilter) -> EnvFilter,
{
    let fmt_layer = tracing_subscriber::fmt::layer().json();

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter_customizer(EnvFilter::from_default_env()));

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
