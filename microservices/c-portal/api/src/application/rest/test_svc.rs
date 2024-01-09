use axum::{Json, Router};
use axum::extract::Path;
use axum::response::{IntoResponse};
use axum::routing::{get};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use crate::infra::{ValidatedPath};

pub fn test_routes() -> Router {
    Router::new()
        .route("/test/:s", get(test))
        .route("/users/:user_id/teams/:team_id", get(pathcustomize))
}

async fn test(ValidatedPath(s): ValidatedPath<i32>) -> String {
    println!("{:?}", s);

    "hello".to_string()
}

async fn pathcustomize(Path(params): Path<Params>) -> impl IntoResponse {
    Json(params)
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    user_id: u32,
    team_id: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct User {
    id: u64,
    name: String,
    created_at: chrono::DateTime<Utc>,
}
