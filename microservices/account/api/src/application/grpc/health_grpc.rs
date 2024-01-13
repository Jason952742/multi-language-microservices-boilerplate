use crate::application::grpc::account_grpc::account_proto::account_server::AccountServer;
use crate::application::grpc::account_grpc::AccountGrpc;
use tonic_health::pb::health_server::{Health, HealthServer};

#[derive(Default)]
pub struct HealthIndicator {}

impl HealthIndicator {
  pub async fn new() -> HealthServer<impl Health + Sized> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<AccountServer<AccountGrpc>>().await;
    health_service
  }
}
