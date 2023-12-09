use std::env;
use tonic::{transport::Server};
use crate::service::health_service::HealthIndicator;
use crate::service::hello_service::MyGreeter;
use crate::service::hello_service::hello_world::greeter_server::GreeterServer;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

    // port
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{port}").parse().unwrap();

    let health_indicator = HealthIndicator::new().await;

    let greeter = MyGreeter::default();

    let enabled = env::var("ENABLED_HELLO").expect("ENABLED_HELLO must be set");
    let optional_service = if enabled == "true" {
        println!("MyGreeter enabled");
        Some(GreeterServer::new(greeter))
    } else {
        println!("MyGreeter disabled");
        None
    };

    println!("HealthServer + GreeterServer listening on {}", addr);

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .add_service(health_indicator)
        .add_optional_service(optional_service)
        .serve(addr)
        .await?;

    Ok(())
}
