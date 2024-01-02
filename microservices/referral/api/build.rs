fn main() {

    let post = "./proto/post.proto";
    let refer_member = "./proto/refer_member.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[post, refer_member], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [post, refer_member]);
}
