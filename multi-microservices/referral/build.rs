fn main() {

    let referral = "./proto/referral.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[referral], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [referral]);
}
