use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::Serialize;
use bcrypt::BcryptError;
use serde_json::json;
use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum JasError {
  // #[error("{0}")]
  // Wither(#[from] WitherError),

  // #[error("{0}")]
  // Mongo(#[from] MongoError),

  #[error("Error parsing ObjectID {0}")]
  ParseObjectID(String),

  #[error("{0}")]
  Authenticate(#[from] AuthenticateError),

  #[error("{0}")]
  BadRequest(#[from] BadRequest),

  #[error("{0}")]
  NotFound(#[from] NotFound),

  #[error("{0}")]
  RunSyncTask(#[from] JoinError),

  #[error("{0}")]
  HashPassword(#[from] BcryptError),
}

impl JasError {
  fn get_codes(&self) -> (StatusCode, u16) {
    match *self {
      // 4XX Errors
      JasError::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
      JasError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
      JasError::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
      JasError::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40004),
      JasError::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40005),
      JasError::Authenticate(AuthenticateError::Locked) => (StatusCode::LOCKED, 40006),

      // 5XX Errors
      JasError::Authenticate(AuthenticateError::TokenCreation) => {
        (StatusCode::INTERNAL_SERVER_ERROR, 5001)
      }
      // JasError::Wither(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5002),
      // JasError::Mongo(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5003),

      JasError::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
      JasError::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
    }
  }

  pub fn bad_request() -> Self {
    JasError::BadRequest(BadRequest {})
  }

  pub fn not_found() -> Self {
    JasError::NotFound(NotFound {})
  }
}

impl IntoResponse for JasError {
  fn into_response(self) -> Response {
    let (status_code, code) = self.get_codes();
    let message = self.to_string();
    let body = Json(json!({ "code": code, "message": message }));

    (status_code, body).into_response()
  }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
  #[error("Wrong authentication credentials")]
  WrongCredentials,
  #[error("Failed to create authentication token")]
  TokenCreation,
  #[error("Invalid authentication credentials")]
  InvalidToken,
  #[error("User is locked")]
  Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(Serialize)]
pub struct PathError {
  pub message: String,
  pub location: Option<String>,
}
