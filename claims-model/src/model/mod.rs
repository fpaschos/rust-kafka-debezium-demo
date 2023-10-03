#[cfg(feature = "proto")]
pub mod proto_conversions;
use serde::{Deserialize, Serialize};

// Re export proto models on "proto" feature
#[cfg(feature = "proto")]
pub mod proto {
    pub use claims_schema::protos::*;
}

// <editor-fold desc="Claim models">
#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ClaimStatus {
    #[default]
    Open,
    Closed,
    Cancelled,
    UnderRevision,
}

#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum IncidentType {
    #[default]
    OtherDamage,
    Collision,
    RoadAssistance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: i32,
    pub claim_no: String,
    pub status: ClaimStatus,
    pub incident_type: IncidentType,
}

// </editor-fold>

// <editor-fold desc="Party models">
#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum PartyType {
    #[default]
    Person,
    Vehicle,
}

#[derive(
    Clone, Copy, Debug, Default, Serialize, Deserialize, strum::Display, strum::EnumString,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")
)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: i32,
    pub claim_id: i32,
    pub r#type: PartyType,
    pub subtype: PartySubtype,
    pub data: PartyData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PartyData {
    #[serde(rename = "PERSON")]
    Person(Person),
    #[serde(rename = "VEHICLE")]
    Vehicle(Vehicle),
}

impl PartyData {
    pub fn r#type(&self) -> PartyType {
        match self {
            PartyData::Vehicle(_) => PartyType::Vehicle,
            PartyData::Person(_) => PartyType::Person,
        }
    }

    pub fn subtype(&self) -> PartySubtype {
        match self {
            PartyData::Vehicle(v) => v.subtype,
            PartyData::Person(v) => v.subtype,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub subtype: PartySubtype,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub subtype: PartySubtype,
    pub reg_no: String,
    pub make: Option<String>,
    pub model: Option<String>,
}

// </editor-fold>
