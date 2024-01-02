#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::net::SocketAddr;
use colored::Colorize;
use sea_orm::DatabaseConnection;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use infra::migration::{Migrator, MigratorTrait};
use application::grpc::health_grpc::HealthIndicator;
use application::grpc::post_grpc::MyServer;
use application::grpc::post_grpc::post_mod::blogpost_server::BlogpostServer;
use application::events::subscribers::MemberSub;
use shared::Config;
use application::grpc::member_grpc::MemberGrpc;
use application::grpc::member_grpc::refer_member_proto::refer_member_server::ReferMemberServer;

mod application;
mod infra;
mod domain;

/// API entry
///
pub async fn start(config: Config, connection: DatabaseConnection) -> anyhow::Result<()> {

    Migrator::up(&connection, None).await?;
    MemberSub::start_subscribe().await?;

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let health_indicator = HealthIndicator::new().await;
    let member_grpc = ReferMemberServer::with_interceptor(MemberGrpc::new(), check_auth);
    let post_grpc = BlogpostServer::new(MyServer { connection });

    tracing::info!("MemberGrpcServer listening on {}", &addr.to_string().color("magenta"));

    Server::builder()
        .trace_fn(|_| tracing::info_span!("Member"))
        .add_service(health_indicator)
        .add_service(member_grpc)
        .add_service(post_grpc)
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
