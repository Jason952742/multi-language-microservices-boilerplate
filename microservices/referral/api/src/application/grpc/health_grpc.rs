use tonic_health::pb::health_server::{Health, HealthServer};
use crate::application::grpc::member_grpc::refer_member_proto::refer_member_server::ReferMemberServer;
use crate::application::grpc::member_grpc::MemberGrpc;

#[derive(Default)]
pub struct HealthIndicator {}

impl  HealthIndicator {
    pub async fn new() -> HealthServer<impl Health + Sized> {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter.set_serving::<ReferMemberServer<MemberGrpc>>().await;
        health_service
    }
}
