use axum::Router;
use axum::routing::{delete, get, post};
use crate::api::rest::endpoints::{add_party, create_claim, fetch_all_claims, remove_party, update_claim, update_party};

pub fn init() -> Router {
    Router::new()
        .route("/claims", get(fetch_all_claims))
        .route("/claims", post(create_claim))
        .route("/claims/:id", post(update_claim))
        .route("/claims/:id/parties", post(add_party))
        .route("/claims/:id/parties/:id", delete(remove_party).post(update_party))

}