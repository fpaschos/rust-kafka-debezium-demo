pub mod protos;

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
            id: 0,
            claim_no: "".into(),
            status: ClaimStatus::OPEN.into(),
            incident_type: IncidentType::OTHER_DAMAGE.into(),
            ..Default::default()
        };
        let serialized = input.write_to_bytes().unwrap();
        let output = protos::claim::Claim::parse_from_bytes(&serialized).unwrap();
        assert_eq!(input, output);

        println!("Claim data {:?}", &output);
    }
}