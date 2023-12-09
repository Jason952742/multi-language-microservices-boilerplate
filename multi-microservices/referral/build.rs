fn main() {

    let hello_world = "./proto/hello_world.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[hello_world], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [hello_world]);
}
