use serde::{Deserialize, Serialize};
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
        let ClaimDb { id, claim_no, incident_type, status } = v;
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
            id:0,
            claim_id,
            r#type: data.r#type(),
            subtype: data.subtype(),
            data: Json(data),
        }
    }
}

impl From<PartyDb> for Party {
    fn from(v: PartyDb) -> Self {
        let PartyDb {id, claim_id, r#type, subtype, data} = v;

        Self {
            id,
            claim_id,
            r#type,
            subtype,
            data: data.0,
        }
    }
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClaimStatus {
    #[default]
    Open,
    Closed,
    Cancelled,
    UnderRevision,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncidentType {
    #[default]
    OtherDamage,
    Collision,
    RoadAssistance,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartyType {
    #[default]
    Person,
    Vehicle,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize, strum::Display, strum::EnumString, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PartySubtype {
    Car,
    Motorbike,

    Owner,
    Beneficiary,
    Driver,
    Passenger,

    #[default]
    Other,
}

// API Common Model
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: i32,
    pub claim_no: String,
    pub status: ClaimStatus,
    pub incident_type: IncidentType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: i32,
    pub claim_id: i32,
    pub r#type: PartyType,
    pub subtype: PartySubtype,
    pub data: PartyData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag="type")]
pub enum PartyData {
    #[serde(rename="PERSON")]
    Person(Person),
    #[serde(rename="VEHICLE")]
    Vehicle(Vehicle),
}

impl PartyData {
    pub fn r#type(&self) -> PartyType {
        match self {
            PartyData::Vehicle(_)=> PartyType::Vehicle,
            PartyData::Person(_) => PartyType::Person,
        }
    }

    pub fn subtype(&self) -> PartySubtype {
        match self {
            PartyData::Vehicle(v)=> v.subtype,
            PartyData::Person(v) => v.subtype,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub subtype: PartySubtype,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub subtype: PartySubtype,
    pub reg_no: String,
    pub make: Option<String>,
    pub model: Option<String>,
}