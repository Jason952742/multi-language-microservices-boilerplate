use tonic_health::pb::health_server::{Health, HealthServer};
use crate::application::grpc::member_grpc::referral_member_proto::referral_member_server::ReferralMemberServer;
use crate::application::grpc::member_grpc::MemberGrpc;

#[derive(Default)]
pub struct HealthIndicator {}

impl  HealthIndicator {
    pub async fn new() -> HealthServer<impl Health + Sized> {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter.set_serving::<ReferralMemberServer<MemberGrpc>>().await;
        health_service
    }
}
