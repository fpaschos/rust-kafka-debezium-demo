use crate::model::{Claim, ClaimStatus, IncidentType, Party, PartyData, PartySubtype, PartyType};
use claims_schema::protos;
use protobuf::{EnumOrUnknown, MessageField};

impl From<IncidentType> for protos::incidentType::IncidentType {
    fn from(value: IncidentType) -> Self {
        match value {
            IncidentType::OtherDamage => Self::OTHER_DAMAGE,
            IncidentType::Collision => Self::COLLISION,
            IncidentType::RoadAssistance => Self::ROAD_ASSISTANCE,
        }
    }
}

impl From<ClaimStatus> for protos::claimStatus::ClaimStatus {
    fn from(value: ClaimStatus) -> Self {
        match value {
            ClaimStatus::Open => Self::OPEN,
            ClaimStatus::Closed => Self::CLOSED,
            ClaimStatus::Cancelled => Self::CANCELLED,
            ClaimStatus::UnderRevision => Self::UNDER_REVISION,
        }
    }
}

impl From<Claim> for protos::claim::Claim {
    fn from(value: Claim) -> Self {
        Self {
            id: value.id,
            claim_no: value.claim_no,
            status: EnumOrUnknown::new(value.status.into()),
            incident_type: EnumOrUnknown::new(value.incident_type.into()),
            ..Default::default()
        }
    }
}

impl From<PartyType> for protos::party::PartyType {
    fn from(value: PartyType) -> Self {
        match value {
            PartyType::Person => Self::PERSON,
            PartyType::Vehicle => Self::VEHICLE,
        }
    }
}

impl From<PartySubtype> for protos::party::PartySubtype {
    fn from(value: PartySubtype) -> Self {
        match value {
            PartySubtype::Car => Self::CAR,
            PartySubtype::Motorbike => Self::MOTORBIKE,
            PartySubtype::Owner => Self::OWNER,
            PartySubtype::Beneficiary => Self::BENEFICIARY,
            PartySubtype::Driver => Self::DRIVER,
            PartySubtype::Passenger => Self::PASSENGER,
            PartySubtype::Other => Self::OTHER,
        }
    }
}

use protos::party::party_data::Data;
use protos::party::Person;
use protos::party::Vehicle;
impl From<PartyData> for protos::party::PartyData {
    fn from(value: PartyData) -> Self {
        match value {
            PartyData::Person(person) => Self {
                data: Some(Data::Person(Person {
                    subtype: EnumOrUnknown::new(person.subtype.into()),
                    name: person.name,
                    ..Default::default()
                })),
                ..Default::default()
            },
            PartyData::Vehicle(vehicle) => Self {
                data: Some(Data::Vehicle(Vehicle {
                    subtype: EnumOrUnknown::new(vehicle.subtype.into()),
                    reg_no: vehicle.reg_no,
                    make: vehicle.make.unwrap_or_default(),
                    model: vehicle.model.unwrap_or_default(),
                    ..Default::default()
                })),
                ..Default::default()
            },
        }
    }
}

impl From<Party> for protos::party::Party {
    fn from(value: Party) -> Self {
        Self {
            id: value.id,
            claim_id: value.claim_id,
            type_: EnumOrUnknown::new(value.r#type.into()),
            subtype: EnumOrUnknown::new(value.subtype.into()),
            data: MessageField::some(value.data.into()),
            ..Default::default()
        }
    }
}
