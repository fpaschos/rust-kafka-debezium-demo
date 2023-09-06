use axum::{Extension, Json};
use axum::extract::Path;
use crate::api::rest::resources::{CreateClaim, UpdateClaim};
use crate::common::api::ApiContext;
use crate::common::error::AppError;
use crate::common::error::DbError::NotFound;
use crate::db;
use crate::model::{Claim, ClaimDb};

pub async fn fetch_all_claims(Extension(context): Extension<ApiContext>) -> Result<Json<Vec<Claim>>, AppError> {
    let entities = db::claims::fetch_all(&context.db).await?;
    let claims: Vec<_> = entities.into_iter().map(|e| { e.try_into().unwrap() }).collect();
    Ok(claims.into())
}

// TODO validation
pub async fn update_claim(
    Extension(context): Extension<ApiContext>,
    Path(id): Path<i64>,
    Json(update_claim): Json<UpdateClaim>,
) -> Result<Json<Claim>, AppError> {
    let UpdateClaim { status, involved } = update_claim;

    let mut tx = context.db.begin().await?;
    let mut entity = db::claims::fetch_one(&mut *tx, id)
        .await?
        .ok_or(AppError::DbError(NotFound))?;
    entity.status = status;
    entity.involved = sqlx::types::Json(involved);
    let entity = db::claims::update(&mut tx, entity).await?;
    tx.commit().await?;

    let claim: Claim = entity.try_into()?;
    Ok(claim.into())
}


// TODO validate
pub async fn create_claim(
    Extension(context): Extension<ApiContext>,
    Json(create_claim): Json<CreateClaim>,
) -> Result<Json<Claim>, AppError> {
    let CreateClaim { involved } = create_claim;


    // Create a new entity via a new transaction
    let mut tx = context.db.begin().await?;
    let entity = ClaimDb::new(involved);
    let entity = db::claims::create(&mut tx, entity).await?;
    tx.commit().await?;

    // Return the new created claim
    let claim: Claim = entity.try_into()?;
    Ok(claim.into())
}

