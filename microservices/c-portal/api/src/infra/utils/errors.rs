use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use serde_json::json;
use tokio::task::JoinError;
use shared::bson;
use shared::mongodb::error::Error as MongoError;


#[derive(thiserror::Error, Debug)]
pub enum CustomError {
  #[error("{0}")]
  Mongo(#[from] MongoError),

  #[error("Error parsing ObjectID: {0}")]
  ParseObjectID(String),

  #[error("{0}")]
  SerializeMongoResponse(#[from] bson::de::Error),

  #[error("{0}")]
  Authenticate(#[from] AuthenticateError),

  #[error("{0}")]
  BadRequest(#[from] BadRequest),

  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),

  #[error(transparent)]
  AxumFormRejection(#[from] FormRejection),

  #[error("{0}")]
  NotFound(#[from] NotFound),

  #[error("{0}")]
  RunSyncTask(#[from] JoinError),

  #[error("{0}")]
  HashPassword(#[from] BcryptError),

  #[error("Error version {0}")]
  BadVersion(String),

  #[error("Error path {0}")]
  BadPath(String),
}

impl CustomError {
  fn get_codes(&self) -> (StatusCode, u16) {
    match *self {
      // 4XX Errors
      CustomError::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
      CustomError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
      CustomError::ValidationError(_) => (StatusCode::BAD_REQUEST, 40003),
      CustomError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, 40004),
      CustomError::BadVersion(_) => (StatusCode::BAD_REQUEST, 40005),
      CustomError::BadPath(_) => (StatusCode::BAD_REQUEST, 40006),
      CustomError::NotFound(_) => (StatusCode::NOT_FOUND, 40403),
      CustomError::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40104),
      CustomError::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40105),
      CustomError::Authenticate(AuthenticateError::Locked) => (StatusCode::LOCKED, 40106),

      // 5XX Errors
      CustomError::Authenticate(AuthenticateError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
      // CustomError::Authorisation(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50002),
      CustomError::Mongo(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50003),
      CustomError::SerializeMongoResponse(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50004),
      CustomError::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50005),
      CustomError::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50006),
    }
  }

  pub fn bad_request() -> Self {
    CustomError::BadRequest(BadRequest {})
  }

  pub fn not_found() -> Self {
    CustomError::NotFound(NotFound {})
  }
}

impl IntoResponse for CustomError {
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



#[derive(Debug)]
pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  TokenCreation,
  InvalidToken,
}


impl IntoResponse for AuthError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
      AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
      AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
      AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
    };
    let body = Json(json!({
            "error": error_message,
        }));
    (status, body).into_response()
  }
}

pub struct AuthorisationError;