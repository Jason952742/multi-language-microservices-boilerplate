fn main() {
  let helloworld = "./proto/helloworld.proto";
  let eventflow = "./proto/eventflow.proto";
  let referral_member = "./proto/referral_member.proto";
  let member = "./proto/member.proto";
  let account = "./proto/account.proto";
  tonic_build::configure()
    .build_server(true)
    .compile(&[helloworld, eventflow, referral_member, member, account], &[".", "proto"])
    .unwrap_or_else(|e| panic!("protobuf compile error: {:?}", e));

  println!("cargo:rerun-if-changed={:?}", [eventflow, referral_member, member, account]);
}
