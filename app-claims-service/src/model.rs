use serde::{Deserialize, Serialize};
use sqlx::types::Json;

// Entities
#[derive(sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct ClaimDb {
    pub id: i64,
    pub status: ClaimStatus,
    pub involved: Json<Party>,
}

impl ClaimDb {
    pub fn new(involved: Party) -> Self {
        Self { id: 0, status: ClaimStatus::default(), involved: Json(involved) }
    }
}

impl TryFrom<ClaimDb> for Claim {
    type Error = anyhow::Error;

    fn try_from(value: ClaimDb) -> Result<Self, Self::Error> {
        let ClaimDb { id, status, involved } = value;
        Ok(Claim {
            id,
            status,
            involved: involved.0,
        })
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all="SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClaimStatus {
    #[default]
    Open,
    Closed,
    CannotAccess
}

// API Common Model
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: i64,
    pub status: ClaimStatus,
    pub involved: Party,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub first_name: String,
    pub last_name: String,
}
