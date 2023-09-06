use serde::{Deserialize, Serialize};
use crate::model::{ClaimStatus, Party};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClaim {
    pub involved: Party,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClaim {
    pub status: ClaimStatus,
    pub involved: Party,
}