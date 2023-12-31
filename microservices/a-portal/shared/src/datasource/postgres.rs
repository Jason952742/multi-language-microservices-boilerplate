use colored::Colorize;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::{info, log};

#[derive(Debug)]
pub struct PgPool;

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

impl PgPool {
    pub async fn conn() -> &'static DatabaseConnection {
        CONN
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let host = env::var("PG_HOST").expect("PG_HOST must be set");
                let port: u16 = env::var("PG_PORT").expect("PG_PORT must be set").parse().unwrap();
                let username = env::var("PG_USERNAME").expect("PG_USERNAME must be set");
                let password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
                let database = env::var("PG_DATABASE").expect("PG_DATABASE must be set");
                let url = format!("postgres://{username}:{password}@{host}:{port}/{database}");
                let connection = get_connection(url).await;
                info!("{}", "POSTGRES CONNECTED!".color("magenta"));
                connection
            }).await
    }
}

// The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
// Since we're using the default superuser we don't have to worry about this too much,
// although we should leave some connections available for manual access.
//
// If you're deploying your application with multiple replicas, then the total
// across all replicas should not exceed the Postgres connection limit.
async fn get_connection(database_url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) // open/close sql log
        .sqlx_logging_level(log::LevelFilter::Info); // default Info;
    Database::connect(opt).await.expect("Postgres connection failed")
}
