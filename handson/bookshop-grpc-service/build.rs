use std::env;
use std::path::PathBuf;

fn main() {
    let proto_file = "./proto/bookshop.proto";
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(output_dir.join("service_descriptor.bin"))
        .out_dir("./src")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("{} compiled.", proto_file);
}
