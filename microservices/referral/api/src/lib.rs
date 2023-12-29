#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::env;
use tokio::sync::mpsc;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use shared::datasource::postgres::PgPool;
use crate::infra::migration::{Migrator, MigratorTrait};
pub use application::grpc::echo_grpc::{EchoServer};
use application::grpc::health_grpc::HealthIndicator;
use application::grpc::hello_grpc::MyGreeter;
use application::grpc::hello_grpc::hello_world::greeter_server::GreeterServer;
use application::grpc::post_grpc::MyServer;
use application::grpc::post_grpc::post_mod::blogpost_server::BlogpostServer;
pub use application::grpc::hello_grpc::hello_world;
pub use application::grpc::echo_grpc::pb;
use application::events::subscribers::MemberSub;
use shared::consul_api;

mod application;
mod infra;
mod domain;

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // env
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
    // will be written to stdout.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_test_writer()
        .init();

    // port
    let addrs = ["0.0.0.0:50051", "0.0.0.0:50052", "0.0.0.0:50053"];
    let cs = consul_api::Consul::new(consul_api::ConsulOption::default()).unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel();

    for addr in &addrs {
        let saddr = addr.parse()?;
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

        let echo = EchoServer { addr: saddr };
        let echo_service = pb::echo_server::EchoServer::with_interceptor(echo, check_auth);

        // establish database connection
        let connection = PgPool::conn().await.clone();
        Migrator::up(&connection, None).await?;
        let hello_server = MyServer { connection };
        let post_service = BlogpostServer::new(hello_server);

        println!("HealthServer + GreeterServer listening on {}", addr);

        tracing::info!(message = "Starting server.", %addr);

        let serve = Server::builder()
            .add_service(health_indicator)
            .add_service(echo_service)
            .add_optional_service(optional_service)
            .add_service(post_service)
            .serve(saddr);

        // register consul service
        register(&cs, addr).await;

        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });
    }

    MemberSub::start_subscribe().await?;

    tokio::spawn(async move {
        cs.discover_service().await.expect("discover_service failed");
        // tokio::time::sleep(Duration::from_secs(5)).await;

    });

    rx.recv().await;


    Ok(())
}

pub async fn register(cs: &consul_api::Consul, addr: &str) {
    // register consul service
    let addrs: Vec<&str> = addr.split(":").collect();
    let port: i32 = addrs[1].parse().unwrap();

    let reg = consul_api::Registration::simple(consul_api::ServiceName::MuReferral, "127.0.0.1", port);
    cs.register(&reg).await.unwrap();
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
