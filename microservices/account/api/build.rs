fn main() {

    let account = "./proto/account.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[account], &[".", "proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

    println!("cargo:rerun-if-changed={:?}", [account]);
}
