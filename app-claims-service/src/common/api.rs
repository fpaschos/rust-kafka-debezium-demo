use std::sync::Arc;
use serde_json::json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
}

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}