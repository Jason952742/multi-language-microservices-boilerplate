use tonic_health::pb::health_server::{Health, HealthServer};
use crate::application::grpc::eventflow_grpc::eventflow_proto::eventflow_server::EventflowServer;
use crate::application::grpc::eventflow_grpc::EventflowGrpc;

#[derive(Default)]
pub struct HealthIndicator {}

impl  HealthIndicator {
    pub async fn new() -> HealthServer<impl Health + Sized> {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter.set_serving::<EventflowServer<EventflowGrpc>>().await;
        health_service
    }
}
