use proto_convert::derive::ProtoConvert;
use proto_convert::{uuid_as_bytes, uuid_as_string, ProtoConvert, ProtoScalar};
use uuid::Uuid;

mod proto;
#[derive(Debug, ProtoConvert, Eq, PartialEq)]
#[proto_convert(source = "proto::EntityUuids")]
struct EntityUuids {
    #[proto_convert(scalar, with = "uuid_as_string")]
    uuid_str: Uuid,
    #[proto_convert(scalar, with = "uuid_as_string")]
    opt_uuid_str: Option<Uuid>,
    #[proto_convert(scalar, with = "uuid_as_bytes")]
    uuid_bytes: Uuid,
    #[proto_convert(scalar, with = "uuid_as_bytes")]
    opt_uuid_bytes: Option<Uuid>,
}

#[test]
fn entity_round_trip() {
    let original = EntityUuids {
        uuid_str: Uuid::new_v4(),
        opt_uuid_str: Some(Uuid::new_v4()),
        uuid_bytes: Uuid::new_v4(),
        opt_uuid_bytes: Some(Uuid::new_v4()),
    };

    let p = original.to_proto();
    let tested = EntityUuids::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_round_trip() {
    let original = proto::EntityUuids {
        uuid_str: Uuid::new_v4().to_string(),
        opt_uuid_str: Uuid::new_v4().to_string(),
        uuid_bytes: Uuid::new_v4().as_bytes().to_vec(),
        opt_uuid_bytes: Uuid::new_v4().as_bytes().to_vec(),
        ..Default::default()
    };

    let e = EntityUuids::from_proto(original.clone()).unwrap();
    assert!(e.opt_uuid_str.is_some());
    assert!(e.opt_uuid_bytes.is_some());

    let tested = e.to_proto();

    assert_eq!(tested, original);
}

#[test]
fn entity_optional_missing_round_trip() {
    let original = EntityUuids {
        uuid_str: Uuid::new_v4(),
        opt_uuid_str: None,
        uuid_bytes: Uuid::new_v4(),
        opt_uuid_bytes: None,
    };

    let p = original.to_proto();
    let tested = EntityUuids::from_proto(p).unwrap();

    assert_eq!(tested, original);
}

#[test]
fn proto_entity_optional_missing_round_trip() {
    let original = proto::EntityUuids {
        uuid_str: Uuid::new_v4().to_string(),
        uuid_bytes: Uuid::new_v4().as_bytes().to_vec(),
        ..Default::default()
    };

    let e = EntityUuids::from_proto(original.clone()).unwrap();
    assert!(e.opt_uuid_str.is_none());
    assert!(e.opt_uuid_bytes.is_none());

    let tested = e.to_proto();

    assert_eq!(tested, original);
}
