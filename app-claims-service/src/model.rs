use serde::{Deserialize, Serialize};
use sqlx::types::Json;

// Entities
#[derive(sqlx::FromRow)]
pub struct ClaimDb {
    pub id: i64,
    pub involved: Json<Party>,
}

impl ClaimDb {
    pub fn new(involved: Party) -> Self {
        Self { id: 0, involved: Json(involved) }
    }
}

impl TryFrom<ClaimDb> for Claim {
    type Error = anyhow::Error;

    fn try_from(value: ClaimDb) -> Result<Self, Self::Error> {
        let ClaimDb { id, involved } = value;
        Ok(Claim {
            id,
            involved: involved.0,
        })
    }
}

// API Common Model
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: i64,
    pub involved: Party,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub first_name: String,
    pub last_name: String,
}
