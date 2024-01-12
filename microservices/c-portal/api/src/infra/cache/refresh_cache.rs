use shared::datasource::dragonfly::DragonflyPool;
use shared::redis::{AsyncCommands, RedisError};
use uuid::Uuid;
use crate::domain::entities::cache_token::{CacheRefreshToken};

pub async fn get_refresh_token(user_id: Uuid) -> Result<CacheRefreshToken, RedisError> {
    let client = DragonflyPool::client_02().await;
    let mut con = client.get_async_connection().await?;
    let key = format!("RT-{:?}", user_id);

    let access_token: String = con.hget(&key, "access_token").await?;
    let refresh_token: String = con.hget(&key, "refresh_token").await?;

    let res = CacheRefreshToken { access_token, refresh_token };

    Ok(res)
}

pub async fn set_refresh_token(user_id: Uuid, token: CacheRefreshToken, expire_in: i64) -> Result<(), RedisError> {
    let client = DragonflyPool::client_02().await;
    let mut con = client.get_async_connection().await?;
    let key = format!("RT-{:?}", &user_id);

    con.hset(&key, "access_token", token.access_token).await?;
    con.hset(&key, "refresh_token", token.refresh_token).await?;

    con.expire(&key, expire_in).await?;

    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}