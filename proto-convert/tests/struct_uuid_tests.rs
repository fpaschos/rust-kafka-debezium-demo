use proto_convert::derive::ProtoConvert;
use proto_convert::{ProtoConvert, ProtoConvertScalar};
use uuid::Uuid;

mod proto;
// #[derive(Debug, ProtoConvert, Eq, PartialEq)]
// #[proto_convert(source = "proto::EntityUuids")]
// struct EntityUuids {
//     #[proto_convert(scalar, with = "uuid_as_string")]
//     uuid_str: Uuid,
//     #[proto_convert(scalar, with = "uuid_as_string")]
//     opt_uuid_str: Option<Uuid>,
//     #[proto_convert(scalar, with = "uuid_as_bytes")]
//     uuid_bytes: Uuid,
//     #[proto_convert(scalar, with = "uuid_as_bytes")]
//     opt_uuid_bytes: Uuid,
// }
