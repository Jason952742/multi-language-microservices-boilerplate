#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::net::SocketAddr;
use colored::Colorize;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use application::grpc::health_grpc::HealthIndicator;
use shared::config::Config;
use application::grpc::eventflow_grpc::EventflowGrpc;
use crate::application::grpc::eventflow_grpc::eventflow_proto::eventflow_server::EventflowServer;
use crate::infra::migration::Migrator;

mod application;
mod infra;
mod domain;

/// API entry
///
pub async fn start(config: Config) -> anyhow::Result<()> {
    // database initialization
    Migrator::migrations().await.expect("database initialization failed");

    // Grpc Service
    let health_indicator = HealthIndicator::new().await;
    let member_grpc = EventflowServer::with_interceptor(EventflowGrpc::new(), check_auth);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("EventFlowGrpcServer listening on {}", &addr.to_string().color("magenta"));
    Server::builder()
        .trace_fn(|_| tracing::info_span!("EventFlow"))
        .add_service(health_indicator)
        .add_service(member_grpc)
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer JaXmn2586KvTz".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
