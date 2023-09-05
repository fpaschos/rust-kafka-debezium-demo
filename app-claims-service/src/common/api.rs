use serde_json::json;
use axum::response::IntoResponse;

pub async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}