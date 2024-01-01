use std::env;
use scylla::{Session, SessionBuilder};
use tokio::sync::OnceCell;
use tracing::info;
use colored::Colorize;

pub struct ScyllaPool;

static SESSION: OnceCell<Session> = OnceCell::const_new();

impl ScyllaPool {
    pub async fn connection() -> &'static Session {
        SESSION
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let uri = env::var("SCYLLA_URI").expect("SCYLLA_URI must be set");
                let session: Session = SessionBuilder::new().known_node(uri).build().await.expect("Scylladb connection failed");
                info!("{}", "SCYLLA CONNECTED".color("magenta"));
                session
            })
            .await
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use scylla::{IntoTypedRows};

    let session = ScyllaPool::connection().await;

    if let Some(rows) = session.query("SELECT a, b, c FROM ks.t", &[]).await?.rows {
        for row in rows.into_typed::<(i32, i32, String)>() {
            let (a, b, c) = row?;
            println!("a, b, c: {}, {}, {}", a, b, c);
        }
    }

    Ok(())
}