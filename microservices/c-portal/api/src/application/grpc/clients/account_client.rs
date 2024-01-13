use crate::application::grpc::account_client::account_proto::account_client::AccountClient;
use crate::application::grpc::account_client::account_proto::{AccountListReply, UserId};
use crate::infra::discovery;
use http::uri::Uri;
use shared::consul_api::ServiceName;
use std::str::FromStr;
use std::time::Duration;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::{transport::Channel, Request};
use tower::timeout::Timeout;
use uuid::Uuid;

pub mod account_proto {
  tonic::include_proto!("account");
}

async fn get_channel() -> Result<(MetadataValue<Ascii>, Timeout<Channel>), Box<dyn std::error::Error>> {
  let srv_addr = discovery(ServiceName::MuAccount.to_string().as_str()).await?;
  let uri = Uri::from_str(&srv_addr).expect("Failed to parse URI");
  let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();
  let channel = Channel::builder(uri).connect().await?;
  let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));
  Ok((token, timeout_channel))
}

// #[tracing::instrument]
pub async fn get_account(user_id: Uuid) -> Result<AccountListReply, Box<dyn std::error::Error>> {
  let (token, timeout_channel) = get_channel().await?;
  let mut client = AccountClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
    let token = token.clone();
    req.metadata_mut().insert("authorization", token);
    Ok(req)
  });

  let response = client.get_accounts_by_user_id(Request::new(UserId { id: user_id.to_string() })).await?;
  Ok(response.into_inner())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let user_id = Uuid::from_str("78ffb767-217f-41e8-8221-23a565d0fea9").unwrap();
  let result = get_account(user_id).await?;

  println!("{:?}", result);

  Ok(())
}
