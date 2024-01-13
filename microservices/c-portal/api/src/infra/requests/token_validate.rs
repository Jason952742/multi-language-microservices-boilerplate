use crate::application::restful::keycloak_client;
use crate::application::services::token_refresh_svc;
use crate::infra::cache::{refresh_cache, token_cache};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::Utc;
use shared::utils::{AuthError, CustomError};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ValidateToken {
  pub user_id: Uuid,
  pub access_token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for ValidateToken
where
  S: Send + Sync,
{
  type Rejection = CustomError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
      .extract::<TypedHeader<Authorization<Bearer>>>()
      .await
      .map_err(|_| CustomError::Authenticate(AuthError::MissingToken))?;

    let access_token = bearer.token();

    match token_cache::get_token(&access_token).await {
      Ok(cache) => {
        let current_time = Utc::now();
        let expires_date = cache.expires_date;
        let user_id = cache.user_id;

        if current_time < expires_date {
          Ok(ValidateToken { user_id: user_id.clone(), access_token: access_token.to_string() })
        } else {
          match refresh_cache::get_refresh_token(&user_id).await {
            Ok(cache) => {
              let new_token = keycloak_client::get_refresh_token(&cache.refresh_token).await?;

              token_refresh_svc::remove_and_refresh(&user_id, new_token.clone()).await?;

              Ok(ValidateToken { user_id, access_token: new_token.access_token })
            }
            Err(_) => Err(CustomError::Authenticate(AuthError::InvalidToken)),
          }
        }
      }
      Err(_) => Err(CustomError::Authenticate(AuthError::InvalidToken)),
    }
  }
}
