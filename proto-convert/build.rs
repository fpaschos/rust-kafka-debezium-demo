use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("Unable to get OUT_DIR");

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&out_dir)
        .input("tests/proto/entities.proto")
        .include("tests/proto")
        .customize(
            protobuf_codegen::Customize::default()
                .generate_accessors(true)
                .gen_mod_rs(true),
        )
        .run_from_script();
    let mod_file_content = r#"//@generated
pub use self::entities::*; 

mod entities;
"#;
    let mod_file_path = Path::new(&out_dir).join("mod.rs");

    let mut file = fs::File::create(mod_file_path).expect("Unable to create mod.rs file");
    file.write_all(mod_file_content.to_string().as_ref())
        .expect("Unable to write mod.rs file");
}
