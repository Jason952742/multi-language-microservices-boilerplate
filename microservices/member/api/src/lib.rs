#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::net::SocketAddr;
use colored::Colorize;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use infra::migration::{Migrator, MigratorTrait};
use application::grpc::health_grpc::HealthIndicator;
use application::events::subscribers::MemberSub;
use shared::config::Config;
use application::grpc::member_grpc::MemberGrpc;
use shared::datasource::mariadb::MariaPool;
use crate::application::grpc::member_grpc::member_proto::member_server::MemberServer;

mod application;
mod infra;
mod domain;

/// API entry
///
pub async fn start(config: Config) -> anyhow::Result<()> {
    // database initialization
    let connection = MariaPool::conn().await;

    // database migrator
    Migrator::up(*&connection, None).await?;

    // RabbitMQ event subscribe
    MemberSub::start_subscribe().await?;

    // Grpc Service
    let health_indicator = HealthIndicator::new().await;
    let member_grpc = MemberServer::with_interceptor(MemberGrpc::new(), check_auth);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("MemberGrpcServer listening on {}", &addr.to_string().color("magenta"));
    Server::builder()
        .trace_fn(|_| tracing::info_span!("Member"))
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
