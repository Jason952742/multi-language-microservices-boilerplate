fn main() {

    let helloworld = "./proto/helloworld.proto";
    let unaryecho = "./proto/unaryecho.proto";
    let post = "./proto/post.proto";
    let refer_member = "./proto/refer_member.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[helloworld, unaryecho, post, refer_member], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [helloworld, unaryecho, post, refer_member]);
}
