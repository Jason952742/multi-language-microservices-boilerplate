fn main() {

    let helloworld = "./proto/helloworld.proto";
    let unaryecho = "./proto/unaryecho.proto";
    let post = "./proto/post.proto";
    let referral = "./proto/referral_member.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[helloworld, unaryecho, post, referral], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [helloworld, unaryecho, post, referral]);
}
