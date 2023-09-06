use axum::Router;
use axum::routing::{get, post};
use crate::api::rest::endpoints::{create_claim, fetch_all_claims, update_claim};

pub fn init() -> Router {
    Router::new()
        .route("/claims", get(fetch_all_claims))
        .route("/claims", post(create_claim))
        .route("/claims/:id", post(update_claim))
}