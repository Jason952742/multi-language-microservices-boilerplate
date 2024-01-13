use crate::application::restful::keycloak_client;
use crate::application::services::token_refresh_svc;
use crate::infra::dto::user::{PasswordBody, PasswordResponse};
use crate::infra::requests::token_validate::ValidateToken;
use axum::routing::{post, put};
use axum::{Json, Router};
use shared::utils::{CustomError, ValidatedJson, ValidatedPath};

pub fn credential_routes() -> Router {
  Router::new()
    .route("/credential/password", put(change_password))
    .route("/credential/forgot-password", post(forgot_password))
    .route("/credential/reset-password/:reset_token", put(reset_password))
}

async fn change_password(token: ValidateToken, ValidatedJson(payload): ValidatedJson<PasswordBody>) -> Result<Json<PasswordResponse>, CustomError> {
  let admin_token = keycloak_client::get_admin_token().await?;

  let _ = keycloak_client::change_password(&token.user_id, &payload.new_password, &admin_token.access_token).await?;

  let user_token = keycloak_client::get_user_token(&payload.identifier, &payload.new_password).await?;

  let _ = token_refresh_svc::remove_and_refresh(&token.user_id, user_token.clone()).await?;

  let res = PasswordResponse { access_token: user_token.access_token };
  Ok(Json(res))
}

async fn forgot_password(_token: ValidateToken) -> Result<(), CustomError> {
  todo!()
}

async fn reset_password(ValidatedPath(reset_token): ValidatedPath<String>) -> Result<(), CustomError> {
  println!("{}", reset_token);
  todo!()
}
