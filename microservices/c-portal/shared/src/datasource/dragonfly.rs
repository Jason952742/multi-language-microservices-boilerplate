use redis::Client;
use std::env;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct DragonflyPool;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

impl DragonflyPool {
    pub async fn client(db: i32) -> &'static Client {
        CLIENT
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let host = env::var("REDIS_HOST").expect("REDIS_HOST must be set");
                let port = env::var("REDIS_PORT").expect("REDIS_PORT must be set");
                let requirepass = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set");
                let redis_url = format!("redis://:{requirepass}@{host}:{port}/{db}");
                let client = Client::open(redis_url).unwrap();
                client
            })
            .await
    }
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
    // use std::collections::HashMap;

    let client = DragonflyPool::client(15).await;
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

    // con.set("key1", b"foo").await?;

    // redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut con).await?;
    //
    // let result = redis::cmd("MGET").arg(&["key1", "key2"]).query_async(&mut con).await;
    //
    // let hello = Payload { foo: "nima".to_string(), bar: 96 };
    //
    // redis::cmd("HSET").arg(&["myhash", "foo", &hello.foo, "bar", &hello.bar.to_string()]).query_async(&mut con).await?;
    //
    // redis::cmd("HSET").arg(&["my-hash", "foo", &hello.foo]).query_async(&mut con).await?;
    // redis::cmd("HSET").arg(&["my-hash", "bar", &hello.bar.to_string()]).query_async(&mut con).await?;
    //
    // redis::cmd("SET").arg("my_key").arg(42i32).query_async(&mut con).await?;

    // let count: i32 = con.get("my_key").await?;
    // println!("{:?}", count);

    // con.hset("nunu", "foo", "wocao").await?;
    // con.hset("nunu", "bar", 47u32.to_string()).await?;

    // let map: HashMap<String, String> = con.hgetall("nunu").await?;
    // println!("{:}", map.get("foo").unwrap());

    // let payload = Payload { foo: map.get("foo").unwrap().to_string(), bar: map.get("bar").parse().unwrap() };

    // println!("{:?}", payload);

    // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));
    Ok(())
}
