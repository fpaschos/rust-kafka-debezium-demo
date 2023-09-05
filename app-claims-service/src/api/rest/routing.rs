use axum::Router;
use axum::routing::post;
use crate::api::rest::endpoints::create_claim;


pub fn init() -> Router {
    Router::new()
        .route("/claims", post(create_claim))
}