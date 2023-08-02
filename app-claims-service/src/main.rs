use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Json;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

mod config;
mod common;

#[derive(sqlx::FromRow)]
struct ClaimDb {
    id: i64,
    involved: Json<Party>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Party {
    first_name: String,
    last_name: String,
}

async fn insert_claim(db: &PgPool, involved: Party) -> anyhow::Result<i64> {
    let c: (i64, ) = sqlx::query_as(r#"INSERT INTO claim (involved) VALUES ($1) RETURNING id"#)
        .bind(Json(involved))
        .fetch_one(db)
        .await?;

    Ok(c.0)
}

async fn fetch_claims(db: &PgPool) -> anyhow::Result<Vec<ClaimDb>> {
    let cs: Vec<ClaimDb> = sqlx::query_as(r#"SELECT id, involved FROM claim"#)
        .fetch_all(db)
        .await?;
    Ok(cs)
}

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


    start_rest_server(&config.server).await.context("Unable to start web server")?;

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


    // Insert a new claim
    // Fetch all claims
    let new_id = insert_claim(&db, involved).await?;
    tracing::debug!("Claim with id = {new_id} inserted");
    let claims = fetch_claims(&db).await?;
    tracing::debug!("Claims found = {}", claims.len());

    Ok(())
}

pub fn setup_tracing(_config: &config::Log) -> anyhow::Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().json();
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())

}

/// Starts the web server given a config [`config::Server`]
pub async fn start_rest_server(config: &config::Server) -> anyhow::Result<()> {
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let app = Router::new().route("/", get(handle));

    tracing::info!("Rest server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
         .with_graceful_shutdown(common::server::shutdown_signal())
        .await?;

    Ok(())
}

async fn handle() -> &'static str {
    "Ok"
}

fn init_routing() {}
