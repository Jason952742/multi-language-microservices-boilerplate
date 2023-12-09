use std::env;
use tokio::sync::mpsc;
use tonic::{transport::Server};
use crate::service::echo_service::{EchoServer, pb};
use crate::service::health_service::HealthIndicator;
use crate::service::hello_service::MyGreeter;
use crate::service::hello_service::hello_world::greeter_server::GreeterServer;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_test_writer().init();

    // port
    let addrs = ["0.0.0.0:50051", "0.0.0.0:50052"];

    let (tx, mut rx) = mpsc::unbounded_channel();

    for addr in &addrs {
        let addr = addr.parse()?;
        let tx = tx.clone();

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

        let echo = EchoServer { addr };
        println!("HealthServer + GreeterServer listening on {}", addr);

        tracing::info!(message = "Starting server.", %addr);

        let serve = Server::builder()
            .add_service(health_indicator)
            .add_service(pb::echo_server::EchoServer::new(echo))
            .add_optional_service(optional_service)
            .serve(addr);

        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });

    }

    rx.recv().await;

    Ok(())
}
