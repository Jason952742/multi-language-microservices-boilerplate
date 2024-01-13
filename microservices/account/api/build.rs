fn main() {

    let member = "./proto/member.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[member], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [member]);
}
