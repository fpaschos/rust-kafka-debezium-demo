use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use axum::{Extension, Router};
use axum::routing::get;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use crate::common::api::{ApiContext, health};

mod config;
mod common;
mod model;
mod db;
mod service;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load(&"./config/application.yml").context("Unable to load config")?;
    let config = Arc::new(config);
    setup_tracing(&config.log)?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db.url)
        .await
        .context("Unable to connect to database")?;

    sqlx::migrate!()
        .run(&db)
        .await
        .context("Unable to exec db migrations")?;


    start_web_server(&config, &db).await.context("Unable to start web server")?;

    Ok(())
}

// TODO configure app log level (using config) and modules log level
pub fn setup_tracing(_config: &config::Log) -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

/// Starts the web server given a config [`config::Server`]
pub async fn start_web_server(config: &Arc<config::AppConfig>, db: &PgPool) -> anyhow::Result<()> {
    // Initialize context
    let context = ApiContext {
        config: config.clone(),
        db: db.clone(),
    };

    // Initialize routing
    let routing = init_routing(context);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Rest server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routing.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(common::server::shutdown_signal())
        .await?;

    Ok(())
}

/// Merge all routers
fn init_routing(context: ApiContext) -> Router {
    let base_router = Router::new().route("/health", get(health));

    // Initialize rest router
    let rest_router = api::rest::routing::init()
        // Add context extension
        .layer(Extension(context));

    base_router.merge(rest_router)
}
