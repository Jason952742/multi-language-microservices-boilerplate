use std::str::FromStr;
use std::time::Duration;
use http::{uri::{Uri}};
use tower::timeout::Timeout;
use tonic::{service::Interceptor, transport::{Channel}, Request};
use tonic::metadata::{Ascii, MetadataValue};
use shared::consul_api::ServiceName;
use referral_proto::{ReferralCode, MemberReply};
use crate::application::grpc::referral_client::referral_proto::referral_member_client::ReferralMemberClient;
use crate::infra::discovery;

pub mod referral_proto {
    tonic::include_proto!("referral_member");
}

async fn get_channel() -> Result<(MetadataValue<Ascii>, Timeout<Channel>), Box<dyn std::error::Error>> {
    let srv_addr = discovery(ServiceName::MuReferral.to_string().as_str()).await?;
    let uri = Uri::from_str(&srv_addr).expect("Failed to parse URI");
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();
    let channel = Channel::builder(uri).connect().await?;
    let timeout_channel = Timeout::new(channel, Duration::from_millis(1500));
    Ok((token, timeout_channel))
}

// #[tracing::instrument]
pub async fn get_referral(code: String) -> Result<MemberReply, Box<dyn std::error::Error>> {
    let (token, timeout_channel) = get_channel().await?;
    let mut client = ReferralMemberClient::with_interceptor(timeout_channel, move |mut req: Request<()>| {
        let token = token.clone();
        req.metadata_mut().insert("authorization", token);
        Ok(req)
    });

    let response = client.get_member_by_code(Request::new(ReferralCode { code }))
        .await?;
    Ok(response.into_inner())
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = get_referral("N43kHRwcSjSLmUA7duGucA".to_string()).await?;

    println!("{:?}", result);

    Ok(())
}