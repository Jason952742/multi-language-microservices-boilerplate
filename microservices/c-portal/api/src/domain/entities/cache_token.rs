use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// Access token for per request authentication
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheToken {
  pub user_id: Uuid,
  pub expires_date: DateTime<Utc>
}

/// When the access token expires, check if there is a refresh token, and if it has not expired, reacquire it with the refresh token
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheRefreshToken {
  pub access_token: String,
  pub refresh_token: String,
}
