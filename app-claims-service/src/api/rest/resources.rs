use serde::{Deserialize, Serialize};
use crate::model::{ClaimStatus, IncidentType};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClaim {
    pub incident_type: IncidentType,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClaim {
    pub incident_type: IncidentType,
    pub status: ClaimStatus,
}