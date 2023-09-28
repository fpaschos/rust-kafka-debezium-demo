use crate::model::{Claim, ClaimStatus, IncidentType};
use claims_schema::protos;

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
            status: <ClaimStatus as Into<protos::claimStatus::ClaimStatus>>::into(value.status)
                .into(),
            incident_type: <IncidentType as Into<protos::incidentType::IncidentType>>::into(
                value.incident_type,
            )
            .into(),
            ..Default::default()
        }
    }
}
