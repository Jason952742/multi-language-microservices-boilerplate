
use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::response::{IntoResponse};
use axum::routing::{get};
use serde::{Deserialize};
use serde_derive::{Serialize};
use validator::Validate;
use shared::utils::{CustomError, PaginationQuery};

pub fn test_routes() -> Router {
    Router::new()
        .route("/test", get(handler))
        .route("/users/:user_id/teams/:team_id", get(pathcustomize))
}

async fn handler(pagination: PaginationQuery, Query(q): Query<QueryParams>) -> Result<String, CustomError> {
    println!("{:?}", pagination);
    println!("{:?}", q);
    Ok("hello".to_string())
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    _qs: i64
}

// #[serde(default, deserialize_with = "empty_string_as_none")]

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
