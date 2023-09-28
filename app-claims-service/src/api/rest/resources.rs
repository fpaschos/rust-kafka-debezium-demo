use claims_model::model::{ClaimStatus, IncidentType, PartyData};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParty {
    pub data: PartyData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateParty {
    pub data: PartyData,
}
