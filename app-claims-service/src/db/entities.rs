use claims_model::model::{
    Claim, ClaimStatus, IncidentType, Party, PartyData, PartySubtype, PartyType,
};
use sqlx::types::Json;

// Entities
#[derive(Default, sqlx::FromRow)]
pub struct ClaimDb {
    pub id: i32,
    pub status: ClaimStatus,
    pub claim_no: String,
    pub incident_type: IncidentType,
}

impl ClaimDb {
    pub fn new<S: AsRef<str>>(claim_no: S, incident_type: IncidentType) -> Self {
        Self {
            claim_no: claim_no.as_ref().into(),
            incident_type,
            ..Self::default()
        }
    }
}

impl From<ClaimDb> for Claim {
    fn from(v: ClaimDb) -> Self {
        let ClaimDb {
            id,
            claim_no,
            incident_type,
            status,
        } = v;
        Self {
            id,
            claim_no,
            incident_type,
            status,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct PartyDb {
    pub id: i32,
    pub claim_id: i32,
    pub r#type: PartyType,
    pub subtype: PartySubtype,
    pub data: Json<PartyData>,
}

impl PartyDb {
    pub fn new(claim_id: i32, data: PartyData) -> Self {
        Self {
            id: 0,
            claim_id,
            r#type: data.r#type(),
            subtype: data.subtype(),
            data: Json(data),
        }
    }
}

impl From<PartyDb> for Party {
    fn from(v: PartyDb) -> Self {
        let PartyDb {
            id,
            claim_id,
            r#type,
            subtype,
            data,
        } = v;

        Self {
            id,
            claim_id,
            r#type,
            subtype,
            data: data.0,
        }
    }
}

#[derive(Default, sqlx::FromRow)]
pub struct ClaimOutboxEventDb {
    pub id: uuid::Uuid,
    pub aggregatetype: String,
    pub aggregateid: String,
    pub r#type: String,
    pub payload: Vec<u8>,
}
