use std::str::FromStr;
use std::time::Duration;
use http::{uri::{Uri}};
use tower::timeout::Timeout;
use tonic::{transport::{Channel}, Request};
use tonic::metadata::{Ascii, MetadataValue};
use uuid::Uuid;
use shared::consul_api::ServiceName;
use crate::application::grpc::member_client::member_proto::member_client::MemberClient;
use crate::application::grpc::member_client::member_proto::{UserId, MemberReply};
use crate::infra::discovery;

pub mod member_proto {
    tonic::include_proto!("member");
}

async fn get_channel() -> Result<(MetadataValue<Ascii>, Timeout<Channel>), Box<dyn std::error::Error>> {
    let srv_addr = discovery(ServiceName::MuMember.to_string().as_str()).await?;
    let uri = Uri::from_str(&srv_addr).expect("Failed to parse URI");
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();
    let channel = Channel::builder(uri).connect().await?;
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));
    Ok((token, timeout_channel))
}

// #[tracing::instrument]
pub async fn get_member(user_id: Uuid) -> Result<MemberReply, Box<dyn std::error::Error>> {
    let (token, timeout_channel) = get_channel().await?;
    let mut client = MemberClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
        let token = token.clone();
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    });

    let response = client.get_member_by_user_id(Request::new(UserId { id: user_id.to_string() }))
        .await?;
    Ok(response.into_inner())
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_id = Uuid::from_str("fd31ccf0-fab9-466a-8ce8-9f1f5431dbf4").unwrap();
    let result = get_member(user_id).await?;

    println!("{:?}", result);

    Ok(())
}