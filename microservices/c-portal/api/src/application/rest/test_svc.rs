
use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::response::{IntoResponse};
use axum::routing::{get};
use serde::{Deserialize};
use serde_derive::{Serialize};
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


