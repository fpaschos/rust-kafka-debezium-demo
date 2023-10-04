use proto_convert::ProtoConvert;

mod proto;

#[derive(Debug, ProtoConvert)]
struct Entity {
    pub id: u32,
    pub nonce: i32,
    pub name: String,
}

#[test]
fn entity_roundtrip() {
    let _e = Entity {
        id: 1,
        nonce: 0,
        name: "Foo".into(),
    };
}
