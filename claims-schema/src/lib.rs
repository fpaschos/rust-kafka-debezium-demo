pub mod protos;

pub const SCHEMA_NAME_CLAIM: &str = "Claim";

pub const RAW_SCHEMA_CLAIM: &str = include_str!("../resources/protos/claim.proto");


pub const SCHEMA_NAME_CLAIM_STATUS: &str = "ClaimStatus";

pub const RAW_SCHEMA_CLAIM_STATUS: &str = include_str!("../resources/protos/claimStatus.proto");


pub const SCHEMA_NAME_INCIDENT_STATUS: &str = "IncidentStatus";

pub const RAW_SCHEMA_INCIDENT_STATUS: &str = include_str!("../resources/protos/incidentType.proto");

#[cfg(test)]
mod tests {
    use protobuf::Message;

    use protos::{
        claimStatus::ClaimStatus,
        incidentType::IncidentType
    };

    use crate::protos;

    #[test]
    fn claim_serialize_deserialize() {
        let input = protos::claim::Claim {
            id: 1,
            claim_no: "TRG1000".into(),
            status: ClaimStatus::OPEN.into(),
            incident_type: IncidentType::OTHER_DAMAGE.into(),
            ..Default::default()
        };
        let serialized = input.write_to_bytes().unwrap();
        let output = protos::claim::Claim::parse_from_bytes(&serialized).unwrap();
        assert_eq!(input, output);

        // println!("Claim data {:?}", &output);
    }
}