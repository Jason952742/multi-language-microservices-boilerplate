fn main() {

    let helloworld = "./proto/helloworld.proto";
    let eventflow = "./proto/eventflow.proto";
    tonic_build::configure()
        .build_server(true)
        .compile(&[helloworld, eventflow], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [helloworld]);
}
