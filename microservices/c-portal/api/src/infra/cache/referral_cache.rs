use std::str::FromStr;
use uuid::Uuid;
use shared::datasource::dragonfly::DragonflyPool;
use shared::redis::{AsyncCommands, RedisError};
use futures::prelude::*;

pub async fn get_referral(code: &str) -> Result<Option<Uuid>, RedisError> {
    let client = DragonflyPool::client(1).await;
    let mut con = client.get_async_connection().await?;
    let key = format!("RF-{}", code);

    let result: Option<String> = con.get(&key).await?;

    match result {
        Some(id) => {
            let u: Uuid = Uuid::from_str(&id).unwrap();
            Ok(Some(u))
        }
        None => Ok(None),
    }

}

pub async fn set_referral(code: &str, user_id: Uuid) -> Result<Uuid, RedisError> {
    let client = DragonflyPool::client(1).await;
    let mut con = client.get_async_connection().await?;
    let key = format!("RF-{}", code);

    con.set(&key, user_id.to_string()).await?;
    con.expire(&key, 60 * 60 * 24 * 30).await?;

    Ok(user_id)
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = set_referral("ddd", Uuid::default()).await?;

    let oid = get_referral("ddd").await?;

    println!("{:?}", oid);

    Ok(())
}