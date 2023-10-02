use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use crate::common::api::{health, ApiContext};
use crate::config::AppConfig;
use crate::service::event_service::EventService;
use anyhow::Context;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};

mod api;
mod common;
mod config;
mod db;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: AppConfig =
        claims_core::config::load(&"./config/application.yml").context("Unable to load config")?;
    let config = Arc::new(config);
    claims_core::tracing::setup_tracing(&config.log)?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .test_before_acquire(true)
        .acquire_timeout(Duration::from_secs(5))
        // This allows us to select which schema to use
        // see: https://docs.rs/sqlx/latest/sqlx/pool/struct.PoolOptions.html#method.after_connect
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // When directly invoking `Executor` methods,
                // it is possible to execute multiple statements with one call.
                conn.execute("SET search_path = 'claims';").await?;
                Ok(())
            })
        })
        .connect(&config.db.url)
        .await
        .context("Unable to connect to database")?;

    sqlx::migrate!()
        .run(&db)
        .await
        .context("Unable to exec db migrations")?;

    start_web_server(config, &db)
        .await
        .context("Unable to start web server")?;

    Ok(())
}

/// Starts the web server given a config [`config::Server`]
pub async fn start_web_server(config: Arc<AppConfig>, db: &PgPool) -> anyhow::Result<()> {
    // Initialize context
    let context = ApiContext {
        config: config.clone(),
        db: db.clone(),
        events: EventService::new(&config.schema_registry.url),
    };

    // Initialize routing
    let routing = init_routing(context);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Rest server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routing.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(claims_core::shutdown::shutdown_signal())
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
