use protobuf_codegen::Codegen;
use std::fs;
use std::io::Write;
use std::path::Path;

// For more details about protobuf codegen
// see : https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-examples
fn main() {
    let out_dir = "src/proto";
    Codegen::new()
        .pure()
        .out_dir(out_dir)
        .includes(["resources/proto"])
        .inputs(["resources/proto/claim.proto", "resources/proto/party.proto"])
        .customize(
            protobuf_codegen::Customize::default()
                .generate_accessors(true)
                .gen_mod_rs(true),
        )
        .run_from_script();

    let mod_file_content = r#"//@generated
pub mod claim;
pub mod party;

"#;
    let mod_file_path = Path::new(&out_dir).join("mod.rs");

    let mut file = fs::File::create(mod_file_path).expect("Unable to create mod.rs file");
    file.write_all(mod_file_content.to_string().as_ref())
        .expect("Unable to write mod.rs file");
}
