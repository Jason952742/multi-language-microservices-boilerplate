use axum::{Json, Router};
use axum::extract::Path;
use axum::response::{IntoResponse};
use axum::routing::{get, post};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;
use crate::infra::{CustomError, ValidatedPath};
use crate::infra::json_validate::ValidatedJson;

pub fn test_routes() -> Router {
    Router::new()
        .route("/test", post(handler))
        .route("/users/:user_id/teams/:team_id", get(pathcustomize))
}

pub async fn handler(ValidatedJson(value): ValidatedJson<Login>) -> Result<String, CustomError> {
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

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Login {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    #[validate()]
    identifier: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    password:String,
    #[serde(default)]
    remember_me: bool,
}
