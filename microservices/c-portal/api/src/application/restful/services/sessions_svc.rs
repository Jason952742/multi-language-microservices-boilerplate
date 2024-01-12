use std::ops::Add;
use axum::{Json, Router};
use axum::routing::{delete, post};
use chrono::{Duration, Utc};
use shared::utils::{AuthError, CustomResponse, to_uuid, ValidatedPath};
use shared::utils::{CustomError, ValidatedJson};
use crate::application::restful::keycloak_client;
use crate::application::services::user_refresh_svc;
use crate::domain::entities::cache_token::{CacheRefreshToken, CacheToken};
use crate::infra::cache::{refresh_cache, token_cache};
use crate::infra::dto::user::{AuthenticateResponse, AuthorizeBody};

pub fn sessions_routes() -> Router<> {
    Router::new()
        .route("/sessions", post(authenticate))
        .route("/sessions/:id", delete(unauthenticate))
}

async fn authenticate(ValidatedJson(body): ValidatedJson<AuthorizeBody>) -> Result<Json<AuthenticateResponse>, CustomError> {
    match keycloak_client::get_user_token(&body.identifier, &body.password).await {
        Ok(user_token) => {
            let claim = keycloak_client::get_user_by_token(&user_token.access_token).await?;
            let user_id = to_uuid(&claim.sub);

            let user = user_refresh_svc::get_or_refresh(user_id, claim).await?;

            // cache access token
            let _ = token_cache::set_token(
                &user_token.access_token,
                CacheToken {
                    user_id: user_id.clone(),
                    expires_date: Utc::now().add(Duration::seconds(user_token.expires_in)),
                },
                user_token.refresh_expires_in,
            ).await?;
            // cache refresh token
            let _ = refresh_cache::set_refresh_token(
                user_id,
                CacheRefreshToken {
                    access_token: user_token.access_token.clone(),
                    refresh_token: user_token.refresh_token,
                },
                user_token.refresh_expires_in,
            ).await?;


            let res = AuthenticateResponse { user, access_token: user_token.access_token };
            Ok(Json(res))
        }
        Err(_) => Err(CustomError::Authenticate(AuthError::WrongCredentials))
    }
}

async fn unauthenticate(ValidatedPath(id): ValidatedPath<String>) -> Result<CustomResponse<()>, CustomError> {
    let user_id = to_uuid(&id);


    todo!()
}
