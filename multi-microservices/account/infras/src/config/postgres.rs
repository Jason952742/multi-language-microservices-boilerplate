use colored::Colorize;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::log;

#[derive(Debug)]
pub struct PgPool;

static REFERRAL_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

impl PgPool {
    fn get_url(pre: &str, database: &str) -> String {
        dotenvy::dotenv().ok();
        let host = env::var("PG_HOST").expect("PG_HOST must be set");
        let port: u16 = env::var("PG_PORT").expect("PG_PORT must be set").parse().unwrap();
        let username = env::var("PG_USERNAME").expect("PG_USERNAME must be set");
        let password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
        let database = env::var(database).expect("PG_DATABASE must be set");
        format!("{pre}://{username}:{password}@{host}:{port}/{database}")
    }

    pub async fn referral_conn() -> &'static DatabaseConnection {
        REFERRAL_CONN
            .get_or_init(|| async {
                let url = Self::get_url("postgres", "PG_DATABASE");
                get_connection(url).await
            })
            .await
    }

}

async fn get_connection(database_url: String) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) // open/close sql log
        .sqlx_logging_level(log::LevelFilter::Error); // default Info
        // .set_schema_search_path("public".into());
    // let connection = Database::connect(&database_url)
    let connection = Database::connect(opt).await.expect("Database connection failed");
    println!("{}", "Database connection!".color("magenta"));
    connection
}
