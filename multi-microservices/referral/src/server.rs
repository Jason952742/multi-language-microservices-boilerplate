use std::env;
use tonic::{transport::Server};
use crate::service::hello_service::MyGreeter;
use crate::service::hello_service::referral::greeter_server::GreeterServer;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_test_writer().init();

    // port
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{port}").parse().unwrap();

    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
