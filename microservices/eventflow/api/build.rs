fn main() {

    let eventflow = "./proto/eventflow.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[eventflow], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [eventflow]);
}
