use std::collections::HashMap;
use async_trait::async_trait;
use axum::extract::{FromRequest, FromRequestParts, Path, Request};
use axum::http::StatusCode;
use axum::{Form, RequestPartsExt, Router};
use axum::extract::rejection::FormRejection;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::MethodRouter;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use tera::Tera;
use thiserror::Error;
use validator::Validate;

#[derive(Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub kind: String,
    pub message: String,
}

pub fn route(path: &str, method_router: MethodRouter<AppState>) -> Router<AppState> {
    Router::new().route(path, method_router)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync,
        Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
            .into_response()
    }
}

#[derive(Debug)]
pub enum Version {
    V1,
    V2,
    V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}