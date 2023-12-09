
use tonic_health::pb::health_server::{Health, HealthServer};
use crate::service::hello_service::hello_world::greeter_server::GreeterServer;
use crate::service::hello_service::MyGreeter;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct HealthIndicator {}

impl  HealthIndicator {
    pub async fn new() -> HealthServer<impl Health + Sized> {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<GreeterServer<MyGreeter>>()
            .await;
        health_service
    }
}
