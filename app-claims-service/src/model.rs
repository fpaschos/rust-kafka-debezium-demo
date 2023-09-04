use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(sqlx::FromRow)]
pub struct ClaimDb {
    pub id: i64,
    pub involved: Json<Party>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub first_name: String,
    pub last_name: String,
}
