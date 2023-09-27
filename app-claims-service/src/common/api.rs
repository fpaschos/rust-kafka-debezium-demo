use crate::config::AppConfig;
use crate::service::event_service::EventService;
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
    pub events: EventService,
}

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}
