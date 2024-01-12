use axum::{Json, Router};
use axum::routing::{post, put};
use shared::utils::{CustomError, to_uuid, ValidatedJson, ValidatedPath};
use crate::application::restful::keycloak_client;
use crate::application::services::{token_refresh_svc};
use crate::infra::dto::user::{PasswordBody, PasswordResponse};

pub fn credential_routes() -> Router<> {
    Router::new()
        .route("/credential/:id/password", put(change_password))
        .route("/credential/:id/forgot-password", post(forgot_password))
        .route("/credential/reset-password/:token", put(reset_password))
}

async fn change_password(ValidatedPath(id): ValidatedPath<String>, ValidatedJson(payload): ValidatedJson<PasswordBody>) -> Result<Json<PasswordResponse>, CustomError> {
    let user_id = to_uuid(&id);
    let admin_token = keycloak_client::get_admin_token().await?;

    let _ = keycloak_client::change_password(user_id, &payload.new_password, &admin_token.access_token).await?;

    let user_token = keycloak_client::get_user_token(&payload.identifier, &payload.new_password).await?;

    let _ = token_refresh_svc::remove_and_refresh(user_id, user_token.clone()).await?;

    let res = PasswordResponse { access_token: user_token.access_token };
    Ok(Json(res))
}

async fn forgot_password() -> Result<(), CustomError> {
    todo!()
}

async fn reset_password() -> Result<(), CustomError> {
    todo!()
}
