use std::ops::Add;
use chrono::{Duration, Utc};
use uuid::Uuid;
use shared::keycloak_api::{Token};
use shared::utils::{CustomError};
use crate::domain::entities::cache_token::{CacheRefreshToken, CacheToken};
use crate::infra::cache::{refresh_cache, token_cache};

pub async fn remove_and_refresh(user_id: Uuid, user_token: Token) -> Result<(), CustomError> {
    remove_tokens(user_id.clone()).await?;

    // cache access token
    let cache_token = CacheToken {
        user_id: user_id.clone(),
        expires_date: Utc::now().add(Duration::seconds(user_token.expires_in)),
    };
    token_cache::set_token(&user_token.access_token, cache_token, &user_token.refresh_expires_in).await?;

    reset_refresh_token(user_id, user_token).await?;

    Ok(())
}

pub async fn remove_tokens(user_id: Uuid) -> Result<(), CustomError> {
    // remove old access token and refresh token
    match refresh_cache::get_refresh_token(user_id.clone()).await {
        Ok(token) => {
            token_cache::delete_token(&token.access_token).await?;
        }
        Err(_) => ()
    }
    Ok(())
}

pub async fn remove_refresh_token(user_id: Uuid) -> Result<(), CustomError> {
    refresh_cache::delete_refresh_token(user_id.clone()).await?;
    Ok(())
}

pub async fn reset_refresh_token(user_id: Uuid, user_token: Token) -> Result<(), CustomError> {
    // cache refresh token
    let cache_refresh = CacheRefreshToken {
        access_token: user_token.access_token.clone(),
        refresh_token: user_token.refresh_token,
    };
    refresh_cache::set_refresh_token(user_id, cache_refresh, user_token.refresh_expires_in).await?;
    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}