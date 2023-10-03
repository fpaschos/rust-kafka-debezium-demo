use protobuf_codegen::Codegen;

// For more details about protobuf codegen
// see : https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-examples
fn main() {
    Codegen::new()
        .pure()
        .out_dir("src/protos")
        .includes(["resources/protos"])
        .inputs([
            "resources/protos/claim.proto",
            "resources/protos/claimStatus.proto",
            "resources/protos/incidentType.proto",
            "resources/protos/party.proto",
        ])
        // .customize_callback(GenSerde)
        .run_from_script();
}
