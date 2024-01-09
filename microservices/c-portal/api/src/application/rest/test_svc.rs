use axum::{Json, Router};
use axum::extract::Path;
use axum::response::{IntoResponse};
use axum::routing::{get, post};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::infra::{CustomError, ValidatedPath};
use crate::infra::json_validate::ValidatedJson;

pub fn test_routes() -> Router {
    Router::new()
        .route("/test", post(handler))
        .route("/users/:user_id/teams/:team_id", get(pathcustomize))
}

pub async fn handler(ValidatedJson(value): ValidatedJson<Params>) -> Result<String, CustomError> {
    println!("{:?}", value);
    Ok("hello".to_string())
}

async fn pathcustomize(Path(params): Path<Params>) -> impl IntoResponse {
    Json(params)
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    user_id: u32,
    team_id: u32,
}

