use redis::Client;
use std::env;
use colored::Colorize;
use tokio::sync::OnceCell;
use tracing::info;

#[derive(Debug)]
pub struct DragonflyPool;

static CLIENT: OnceCell<Client> = OnceCell::const_new();
static CLIENT_01: OnceCell<Client> = OnceCell::const_new();
static CLIENT_02: OnceCell<Client> = OnceCell::const_new();
static CLIENT_03: OnceCell<Client> = OnceCell::const_new();
static CLIENT_04: OnceCell<Client> = OnceCell::const_new();
static CLIENT_05: OnceCell<Client> = OnceCell::const_new();
static CLIENT_06: OnceCell<Client> = OnceCell::const_new();
static CLIENT_07: OnceCell<Client> = OnceCell::const_new();
static CLIENT_08: OnceCell<Client> = OnceCell::const_new();
static CLIENT_09: OnceCell<Client> = OnceCell::const_new();
static CLIENT_10: OnceCell<Client> = OnceCell::const_new();
static CLIENT_11: OnceCell<Client> = OnceCell::const_new();
static CLIENT_12: OnceCell<Client> = OnceCell::const_new();
static CLIENT_13: OnceCell<Client> = OnceCell::const_new();
static CLIENT_14: OnceCell<Client> = OnceCell::const_new();
static CLIENT_15: OnceCell<Client> = OnceCell::const_new();

impl DragonflyPool {
    pub async fn client() -> &'static Client {
        CLIENT.get_or_init(|| async { get_client(0).await }).await
    }

    pub async fn client_01() -> &'static Client {
        CLIENT_01.get_or_init(|| async { get_client(1).await }).await
    }

    pub async fn client_02() -> &'static Client {
        CLIENT_02.get_or_init(|| async { get_client(2).await }).await
    }

    pub async fn client_03() -> &'static Client {
        CLIENT_03.get_or_init(|| async { get_client(3).await }).await
    }

    pub async fn client_04() -> &'static Client {
        CLIENT_04.get_or_init(|| async { get_client(4).await }).await
    }

    pub async fn client_05() -> &'static Client {
        CLIENT_05.get_or_init(|| async { get_client(5).await }).await
    }

    pub async fn client_06() -> &'static Client {
        CLIENT_06.get_or_init(|| async { get_client(6).await }).await
    }

    pub async fn client_07() -> &'static Client {
        CLIENT_07.get_or_init(|| async { get_client(7).await }).await
    }

    pub async fn client_08() -> &'static Client {
        CLIENT_08.get_or_init(|| async { get_client(8).await }).await
    }

    pub async fn client_09() -> &'static Client {
        CLIENT_09.get_or_init(|| async { get_client(9).await }).await
    }

    pub async fn client_10() -> &'static Client {
        CLIENT_10.get_or_init(|| async { get_client(10).await }).await
    }

    pub async fn client_11() -> &'static Client {
        CLIENT_11.get_or_init(|| async { get_client(11).await }).await
    }

    pub async fn client_12() -> &'static Client {
        CLIENT_12.get_or_init(|| async { get_client(12).await }).await
    }

    pub async fn client_13() -> &'static Client {
        CLIENT_13.get_or_init(|| async { get_client(13).await }).await
    }

    pub async fn client_14() -> &'static Client {
        CLIENT_14.get_or_init(|| async { get_client(14).await }).await
    }

    pub async fn client_15() -> &'static Client {
        CLIENT_15.get_or_init(|| async { get_client(15).await }).await
    }
}

async fn get_client(db: i32) -> Client {
    dotenvy::dotenv().ok();
    let host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    let requirepass = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set");
    let redis_url = format!("redis://:{requirepass}@{host}:{port}/{db}");
    let client = Client::open(redis_url).expect("Dragonfly connection failed");
    info!("{} {}", "DRAGONFLY CONNECTED".color("magenta"), db);
    client
}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct Payload {
        foo: String,
        bar: u32,
    }

    impl From<&[u8]> for Payload {
        fn from(v: &[u8]) -> Self {
            serde_json::from_slice::<Payload>(v).unwrap()
        }
    }

    impl Into<Vec<u8>> for Payload {
        fn into(self) -> Vec<u8> {
            serde_json::to_vec(&json!(self)).expect("Error decoding payload")
        }
    }

    impl Into<String> for Payload {
        fn into(self) -> String {
            serde_json::to_string(&json!(self)).expect("Error decoding payload")
        }
    }

    use redis::AsyncCommands;

    let client = DragonflyPool::client().await;
    let mut con = client.get_async_connection().await?;

    let _ = con.set("hello", "world").await?;
    let result: Option<String> = con.get("hello").await?;

    match result {
        Some(v) => {
            println!("Result: {}", v);
        }
        None => {
            println!("None");
        }
    }

    Ok(())
}
