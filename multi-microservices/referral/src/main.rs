#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::env;
use tokio::sync::mpsc;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use crate::service::echo_service::{EchoServer, pb};
use crate::service::health_service::HealthIndicator;
use crate::service::hello_service::MyGreeter;
use crate::service::hello_service::hello_world::greeter_server::GreeterServer;

mod service;

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // env
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_test_writer().init();

    // port
    let addrs = ["0.0.0.0:50051", "0.0.0.0:50052", "0.0.0.0:50053"];

    let (tx, mut rx) = mpsc::unbounded_channel();

    for addr in &addrs {
        let addr = addr.parse()?;
        let tx = tx.clone();

        let health_indicator = HealthIndicator::new().await;
        let greeter = MyGreeter::default();
        let enabled = env::var("ENABLED_HELLO").expect("ENABLED_HELLO must be set");
        let optional_service = if enabled == "true" {
            println!("MyGreeter enabled");
            // See examples/src/interceptor/client.rs for an example of how to create a
            // named interceptor that can be returned from functions or stored in structs.
            Some(GreeterServer::with_interceptor(greeter, intercept))
        } else {
            println!("MyGreeter disabled");
            None
        };

        let echo = EchoServer { addr };
        let echo_service = pb::echo_server::EchoServer::with_interceptor(echo, check_auth);

        println!("HealthServer + GreeterServer listening on {}", addr);

        tracing::info!(message = "Starting server.", %addr);

        let serve = Server::builder()
            .add_service(health_indicator)
            .add_service(echo_service)
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

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}

/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.
fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);

    // Set an extension that can be retrieved by `say_hello`
    req.extensions_mut().insert(MyExtension {
        some_piece_of_data: "foo".to_string(),
    });

    Ok(req)
}

struct MyExtension {
    some_piece_of_data: String,
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
