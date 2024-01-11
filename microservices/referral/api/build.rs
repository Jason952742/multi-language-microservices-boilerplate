fn main() {

    let referral_member = "./proto/referral_member.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[referral_member], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [referral_member]);
}
