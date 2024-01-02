#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::net::SocketAddr;
use colored::Colorize;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use application::grpc::health_grpc::HealthIndicator;
use application::events::subscribers::MemberSub;
use shared::Config;
use application::grpc::member_grpc::MemberGrpc;
use application::grpc::member_grpc::refer_member_proto::refer_member_server::ReferMemberServer;
use shared::scylladb::ScyllaPool;

mod application;
mod infra;
mod domain;

/// API entry
///
pub async fn start(config: Config) -> anyhow::Result<()> {
    // database initialization
    let _session = ScyllaPool::connection().await;

    // Grpc Service
    let health_indicator = HealthIndicator::new().await;
    let member_grpc = ReferMemberServer::with_interceptor(MemberGrpc::new(), check_auth);

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
