use axum::{Extension, Json};
use crate::api::rest::resources::CreateClaim;
use crate::common::api::ApiContext;
use crate::common::error::AppError;
use crate::db;
use crate::model::{Claim, ClaimDb};

// TODO implement app error
pub async fn create_claim(
    Extension(context): Extension<ApiContext>,
    Json(create_claim): Json<CreateClaim>,
) -> Result<Json<Claim>, AppError> {
    let CreateClaim { involved } = create_claim;
    // TODO validate

    // Create a new entity via a new transaction
    let mut tx = context.db.begin().await?;
    let entity = ClaimDb::new(involved);
    let entity = db::create_claim(&mut tx, entity).await?;
    tx.commit().await?;


    // Return the new created claim
    let claim: Claim = entity.try_into()?;
    Ok(claim.into())
}

pub async fn fetch_all_claims(Extension(context): Extension<ApiContext>) -> Result<Json<Vec<Claim>>, AppError> {
    let entities = db::fetch_claims(&context.db).await?;
    let claims: Vec<_> = entities.into_iter().map(|e| { e.try_into().unwrap() }).collect();
    Ok(claims.into())
}