use std::convert::Infallible;
use axum::Json;
use crate::api::rest::resources::CreateClaim;
use crate::model::{Claim, ClaimDb};

// TODO implement app error
pub async fn create_claim(Json(create_claim): Json<CreateClaim>) -> Result<Json<Claim>, Infallible> {
    let CreateClaim { involved } = create_claim;

    // TODO implement ClaimDb::new
    let entity = ClaimDb {
        id: 0,
        involved: sqlx::types::Json(involved),
    };

    // TODO access the database

    let claim: Claim = entity.try_into()?;
    Ok(claim.into())
}

pub async fn fetch_all_claims() {
    todo!()
}