use core::fmt;
use axum::extract::rejection::{FormRejection, PathRejection, QueryRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use tokio::task::JoinError;
use bson;


#[derive(thiserror::Error, Debug)]
pub enum CustomError {
  #[error("{0}")]
  Mongo(#[from] mongodb::error::Error),

  #[error("Error parsing ObjectID: {0}")]
  ParseObjectID(String),

  #[error("{0}")]
  SerializeMongoResponse(#[from] bson::de::Error),

  #[error("{0}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("{0}")]
  Authenticate(#[from] AuthError),

  // #[error("Error authorisation: {0}")]
  // Authorisation(String),

  #[error("{0}")]
  BadRequest(#[from] BadRequest),

  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),

  #[error(transparent)]
  AxumFormRejection(#[from] FormRejection),

  #[error("{0}")]
  AxumJsonRejection(AxumJsonRejection),

  #[error("{0}")]
  AxumQueryRejection(#[from] QueryRejection),

  #[error("{0}")]
  AxumPathRejection(#[from] PathRejection),

  #[error("{0}")]
  NotFound(#[from] NotFound),

  #[error("{0}")]
  AlreadyExists(#[from] AlreadyExists),

  #[error("{0}")]
  RunSyncTask(#[from] JoinError),

  #[error("{0}")]
  HashPassword(#[from] BcryptError),

  #[error("Error version {0}")]
  BadVersion(String),

  #[error("Error {0}")]
  BoxStdError(#[from] Box<dyn std::error::Error>),

  #[error("Error {0}")]
  UnknownErr(&'static str),
}

impl CustomError {
  fn get_codes(&self) -> (StatusCode, u16) {
    match *self {
      // 4XX Errors
      CustomError::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
      CustomError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
      CustomError::ValidationError(_) => (StatusCode::BAD_REQUEST, 40003),
      CustomError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, 40004),
      CustomError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, 40005),
      CustomError::AxumQueryRejection(_) => (StatusCode::BAD_REQUEST, 40006),
      CustomError::AxumPathRejection(_) => (StatusCode::BAD_REQUEST, 40007),
      CustomError::BadVersion(_) => (StatusCode::BAD_REQUEST, 40008),

      CustomError::NotFound(_) => (StatusCode::NOT_FOUND, 40403),
      CustomError::AlreadyExists(_) => (StatusCode::CONFLICT, 40901),

      CustomError::Authenticate(AuthError::MissingCredentials) => (StatusCode::UNAUTHORIZED, 40102),
      CustomError::Authenticate(AuthError::MissingToken) => (StatusCode::UNAUTHORIZED, 40103),
      CustomError::Authenticate(AuthError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40104),
      CustomError::Authenticate(AuthError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40105),

      // CustomError::Authenticate(AuthError::Locked) => (StatusCode::LOCKED, 42301),
      // CustomError::Authorisation(_) => (StatusCode::FORBIDDEN, 40301),

      // 5XX Errors
      CustomError::Authenticate(AuthError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
      CustomError::ReqwestError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50002),
      CustomError::Mongo(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50003),
      CustomError::SerializeMongoResponse(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50004),
      CustomError::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50005),
      CustomError::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50006),
      CustomError::BoxStdError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50007),
      CustomError::UnknownErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50008),
    }
  }

  pub fn _bad_request() -> Self {
    CustomError::BadRequest(BadRequest {})
  }

  pub fn not_found() -> Self {
    CustomError::NotFound(NotFound {})
  }

  pub fn already_exists() -> Self {
    CustomError::AlreadyExists(AlreadyExists {})
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
pub enum AuthError {
  #[error("Missing authentication credentials")]
  MissingCredentials,
  #[error("Wrong authentication credentials")]
  WrongCredentials,
  #[error("Failed to create authentication token")]
  TokenCreation,
  #[error("Missing authentication token")]
  MissingToken,
  #[error("Invalid authentication token")]
  InvalidToken,
  // #[error("User is locked")]
  // Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(thiserror::Error, Debug)]
#[error("Already exists")]
pub struct AlreadyExists {}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonError {
  pub message: String,
  pub origin: Option<String>
}

#[derive(Debug)]
pub struct AxumJsonRejection(pub Json<JsonError>);

impl fmt::Display for AxumJsonRejection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Axum JSON rejection: {:?}", self.0)
  }
}