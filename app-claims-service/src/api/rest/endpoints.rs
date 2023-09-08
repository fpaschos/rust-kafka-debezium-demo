use axum::{Extension, Json};
use axum::extract::Path;
use crate::api::rest::resources::{CreateClaim, CreateParty, UpdateClaim};
use crate::common::api::ApiContext;
use crate::common::error::AppError;
use crate::common::error::DbError::NotFound;
use crate::{common, db};
use crate::model::{Claim, ClaimDb, Party, PartyDb};
// TODO add proper validation to all update eendpoints
// TODO refactor endpoint internals to be reusable for other apis eg. grpc, graphql etc...
// TODO in case of `create_claim` endpoint provide deterministic unique claim id via sequence?

pub async fn fetch_all_claims(Extension(context): Extension<ApiContext>) -> Result<Json<Vec<Claim>>, AppError> {
    let entities = db::claims::fetch_all(&context.db).await?;
    let claims: Vec<Claim> = entities.into_iter().map(|e| { e.into() }).collect();
    Ok(claims.into())
}

pub async fn update_claim(
    Extension(context): Extension<ApiContext>,
    Path(id): Path<i32>,
    Json(update_claim): Json<UpdateClaim>,
) -> Result<Json<Claim>, AppError> {
    let UpdateClaim { incident_type, status } = update_claim;

    let mut tx = context.db.begin().await?;
    let mut e = db::claims::fetch_one(&mut *tx, id)
        .await?
        .ok_or(AppError::DbError(NotFound))?;

    e.status = status;
    e.incident_type = incident_type;

    let entity = db::claims::update(&mut tx, e).await?;
    tx.commit().await?;

    let claim: Claim = entity.into();
    Ok(claim.into())
}


pub async fn create_claim(
    Extension(context): Extension<ApiContext>,
    Json(create_claim): Json<CreateClaim>,
) -> Result<Json<Claim>, AppError> {
    let CreateClaim { incident_type } = create_claim;
    let claim_no = common::misc::generate_random_string(10);


    // Create a new entity via a new transaction
    let mut tx = context.db.begin().await?;
    let entity = ClaimDb::new(claim_no, incident_type);
    let entity = db::claims::create(&mut tx, entity).await?;
    tx.commit().await?;

    // Return the new created claim
    let claim: Claim = entity.into();
    Ok(claim.into())
}

pub async fn add_party(
    Extension(context): Extension<ApiContext>,
    Path(claim_id): Path<i32>,
    Json(create_party): Json<CreateParty>,
) -> Result<Json<Party>, AppError> {
    let CreateParty { data } = create_party;

    let mut tx = context.db.begin().await?;
    let claim = db::claims::fetch_one(&context.db, claim_id)
        .await?
        .ok_or(AppError::DbError(NotFound))?;
    let party = PartyDb::new(claim.id, data);
    let party = db::parties::create(&mut tx, party).await?;

    tx.commit().await?;

    let party: Party = party.into();
    Ok(party.into())
}


pub async fn remove_party(
    Extension(context): Extension<ApiContext>,
    Path((claim_id, party_id)): Path<(i32,i32)>,
) -> Result<Json<Party>, AppError> {
    let mut tx = context.db.begin().await?;
    // Validate that the claim exists
    db::claims::fetch_one(&context.db, claim_id)
        .await?
        .ok_or(AppError::DbError(NotFound))?;

    let party = db::parties::fetch_one(&context.db, party_id)
        .await?
        .ok_or(AppError::DbError(NotFound))?;
    let party = db::parties::delete(&mut tx, party).await?;

    tx.commit().await?;

    let party: Party = party.into();
    Ok(party.into())
}

//
// pub async fn update_party(
//     Extension(context): Extension<ApiContext>,
//     Path(claim_id): Path<i32>,
//     Path(party_id): Path<i32>,
//     Json(update_party): Json<UpdateParty>,
// ) -> Result<Json<Party>, AppError> {
//     todo!()
// }



