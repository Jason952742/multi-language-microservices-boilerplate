use crate::application::restful::keycloak_client;
use crate::application::services::{token_refresh_svc, user_refresh_svc};
use crate::infra::dto::user::{AuthenticateResponse, AuthorizeBody};
use crate::infra::requests::token_validate::ValidateToken;
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use shared::utils::{to_uuid, AuthError, CustomResponse, CustomResponseBuilder};
use shared::utils::{CustomError, ValidatedJson};

pub fn sessions_routes() -> Router {
  Router::new().route("/sessions", post(authenticate)).route("/sessions", delete(unauthenticate))
}

async fn authenticate(ValidatedJson(body): ValidatedJson<AuthorizeBody>) -> Result<Json<AuthenticateResponse>, CustomError> {
  match keycloak_client::get_user_token(&body.identifier, &body.password).await {
    Ok(user_token) => {
      let claim = keycloak_client::get_user_by_token(&user_token.access_token).await?;
      let user_id = to_uuid(&claim.sub);

      let user = user_refresh_svc::get_or_refresh(user_id, claim).await?;
      let _ = token_refresh_svc::remove_and_refresh(&user_id, user_token.clone()).await?;

      let res = AuthenticateResponse { user, access_token: user_token.access_token };
      Ok(Json(res))
    }
    Err(_) => Err(CustomError::Authenticate(AuthError::WrongCredentials)),
  }
}

async fn unauthenticate(token: ValidateToken) -> Result<CustomResponse<()>, CustomError> {
  println!("{:?}", token);

  let _ = token_refresh_svc::remove_access_token(&token.user_id).await?;
  let _ = token_refresh_svc::remove_refresh_token(&token.user_id).await?;

  let res = CustomResponseBuilder::new().status_code(StatusCode::NO_CONTENT).build();
  Ok(res)
}
