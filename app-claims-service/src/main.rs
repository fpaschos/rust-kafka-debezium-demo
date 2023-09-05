use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use crate::common::api::health;

mod config;
mod common;
mod model;
mod db;
mod service;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load(&"./config/application.yml").context("Unable to load config")?;
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


    start_web_server(&config.server).await.context("Unable to start web server")?;

    Ok(())
}
/*
    // Just check the database connection
    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64, ) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&db)
        .await?;

    assert_eq!(row.0, 150);

    let involved = Party {
        first_name: "Foo".into(),
        last_name: "Bar".into(),
    };
    let claim = ClaimDb {
        id: 0,
        involved: Json(involved),
    };


    // Try transaction
    // Insert a new claim
    // Fetch all claims
    // Revert
    // Fetch all claims
    {
        let mut tx = db.begin().await?;
        let claim  = create_claim(&mut tx,  claim).await?;
        tracing::debug!("Claim with id = {} inserted", claim.id);
        let claims = fetch_claims(&mut *tx).await?;

        tracing::debug!("Claims found = {}", claims.len());
        tracing::debug!("Rolling back transaction");
    }
    let claims = fetch_claims(&db).await?;
    tracing::debug!("Claims found = {}", claims.len());

}

 */

pub fn setup_tracing(_config: &config::Log) -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())

}

/// Starts the web server given a config [`config::Server`]
pub async fn start_web_server(config: &config::Server) -> anyhow::Result<()> {
    // Initialize routing
    let routing = init_routing();

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Rest server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routing.into_make_service_with_connect_info::<SocketAddr>())
         .with_graceful_shutdown(common::server::shutdown_signal())
        .await?;

    Ok(())
}

/// Merge all routers
fn init_routing() -> Router {
    let base_router = Router::new().route("/health", get(health));

    let rest_router = api::rest::routing::init();

    base_router.merge(rest_router)
}
