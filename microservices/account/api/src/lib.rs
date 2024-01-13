#![forbid(unsafe_code)]
#![deny(clippy::all)]

use crate::application::grpc::account_grpc::account_proto::account_server::AccountServer;
use application::events::subscribers::AccountSub;
use application::grpc::account_grpc::AccountGrpc;
use application::grpc::health_grpc::HealthIndicator;
use colored::Colorize;
use infra::migration::{Migrator, MigratorTrait};
use shared::config::Config;
use shared::datasource::mariadb::MariaPool;
use std::net::SocketAddr;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};

mod application;
mod domain;
mod infra;

/// API entry
///
pub async fn start(config: Config) -> anyhow::Result<()> {
  // database initialization
  let connection = MariaPool::conn().await;

  // database migrator
  Migrator::up(*&connection, None).await?;

  // RabbitMQ event subscribe
  AccountSub::start_subscribe().await?;

  // Grpc Service
  let health_indicator = HealthIndicator::new().await;
  let account_grpc = AccountServer::with_interceptor(AccountGrpc::new(), check_auth);

  let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
  tracing::info!("AccountGrpcServer listening on {}", &addr.to_string().color("magenta"));
  Server::builder()
    .trace_fn(|_| tracing::info_span!("Account"))
    .add_service(health_indicator)
    .add_service(account_grpc)
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
