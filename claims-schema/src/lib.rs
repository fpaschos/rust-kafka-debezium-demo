use claims_core::schema_name_impl;

pub mod proto;

use proto::claim::Claim;
use proto::party::Party;

use claims_core::proto_encode::message::SchemaName;

// Needed for correctly using proto_encoder
const CLAIMS_SCHEMA: &str = "claims.schema.";

schema_name_impl!(CLAIMS_SCHEMA, Claim);
schema_name_impl!(CLAIMS_SCHEMA, Party);

#[cfg(test)]
mod tests {

    use protobuf::Message;

    use crate::proto::claim::{Claim, ClaimStatus, IncidentType};

    #[test]
    fn claim_serialize_deserialize() {
        let input = Claim {
            id: 1,
            claim_no: "TRG1000".into(),
            status: ClaimStatus::OPEN.into(),
            incident_type: IncidentType::OTHER_DAMAGE.into(),
            ..Default::default()
        };
        let serialized = input.write_to_bytes().unwrap();
        let output = Claim::parse_from_bytes(&serialized).unwrap();
        assert_eq!(input, output);
    }
}
