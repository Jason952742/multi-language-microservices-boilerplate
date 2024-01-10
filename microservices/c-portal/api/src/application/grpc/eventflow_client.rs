use std::str::FromStr;
use std::time::Duration;
use http::{
    uri::{InvalidUri, Uri},
};
use tower::timeout::Timeout;
use tonic::{
    codegen::InterceptedService,
    service::Interceptor,
    transport::{Channel, Endpoint},
    Request, Status,
};
use tonic::metadata::{Ascii, MetadataValue};
use uuid::Uuid;
use shared::consul_api::ServiceName;
use eventflow_proto::{AccountTransferRequest, AccountTransactionReply, eventflow_server, ListRequest, MemberSubscriptionReply, MemberSubscriptionRequest, TransactionId, TransactionInfo, TransactionListReply, TransactionReply, UserCreatedReply, UserCreateRequest, UserInfo};
use crate::application::grpc::eventflow_client::eventflow_proto::eventflow_client::EventflowClient;
use crate::infra::discovery;

pub mod eventflow_proto {
    tonic::include_proto!("eventflow");
}

async fn get_channel() -> Result<(MetadataValue<Ascii>, Timeout<Channel>), Box<dyn std::error::Error>> {
    let srv_addr = discovery(ServiceName::MuEventFlow.to_string().as_str()).await?;
    let uri = Uri::from_str(&srv_addr).expect("Failed to parse URI");
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();
    let channel = Channel::builder(uri).connect().await?;
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));
    Ok((token, timeout_channel))
}

#[tracing::instrument]
async fn get_transaction_by_id(id: Uuid) -> Result<TransactionReply, Box<dyn std::error::Error>> {
    let (token, timeout_channel) = get_channel().await?;
    let mut client = EventflowClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
        let token = token.clone();
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    });

    let response = client.get_transaction_by_id(Request::new(TransactionId { id: id.to_string() }))
        .await?;
    Ok(response.into_inner())
}

#[tracing::instrument]
pub async fn user_create(user_id: Uuid, user_name: String, referrer_id: Option<String>, referrer_code: Option<String>) -> Result<UserCreatedReply, Box<dyn std::error::Error>> {
    let (token, timeout_channel) = get_channel().await?;
    let mut client = EventflowClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
        let token = token.clone();
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    });

    let response = client.user_create(Request::new(UserCreateRequest { user_id: user_id.to_string(), user_name, referrer_id, referrer_code }))
        .await?;

    Ok(response.into_inner())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::from_str("11112222-e4a9-4fba-bb8d-555566667777").unwrap();
    let result = user_create(id, "wowowo".to_string(), None, None).await?;

    println!("{:?}", result);

    Ok(())
}