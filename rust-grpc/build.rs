fn main() {
    let proto_files = &["./proto/echo_unary.proto", "./proto/echo_streaming.proto"];

    let dirs = &["."];
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./src")
        .compile(proto_files, dirs)
        .unwrap_or_else(|e| panic!("protobuf compile error {}", e));

    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
}
