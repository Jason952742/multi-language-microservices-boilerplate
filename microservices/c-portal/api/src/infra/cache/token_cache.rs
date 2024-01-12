use shared::datasource::dragonfly::DragonflyPool;
use shared::redis::{AsyncCommands, RedisError};
use shared::utils::{to_datetime, to_uuid};
use crate::domain::entities::cache_token::CacheToken;

pub async fn get_token(access_token: &str) -> Result<CacheToken, RedisError> {
    let client = DragonflyPool::client_01().await;
    let mut con = client.get_async_connection().await?;
    let key = format!("AT-{}", access_token);

    let user_id: String = con.hget(&key, "user_id").await?;
    let expires_date: String = con.hget(&key, "expires_date").await?;

    let res = CacheToken {
        user_id: to_uuid(&user_id),
        expires_date: to_datetime(&expires_date),
    };

    Ok(res)

}

pub async fn set_token(access_token: &str, token: CacheToken, expire_in: i64) -> Result<(), RedisError> {
    let client = DragonflyPool::client_01().await;
    let mut con = client.get_async_connection().await?;
    let key = format!("AT-{}", access_token);

    con.hset(&key, "user_id", token.user_id.to_string()).await?;
    con.hset(&key, "expires_date", token.expires_date.to_string()).await?;

    con.expire(&key, expire_in).await?;

    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}