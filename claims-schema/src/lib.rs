use claims_core::schema_name_impl;

pub mod protos;

use protos::claim::Claim;

use claims_core::proto_encode::message::SchemaName;

const CLAIMS_SCHEMA: &str = "claims.schema.";

schema_name_impl!(CLAIMS_SCHEMA, Claim);

#[cfg(test)]
mod tests {
    use protobuf::Message;

    use protos::{claimStatus::ClaimStatus, incidentType::IncidentType};

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
