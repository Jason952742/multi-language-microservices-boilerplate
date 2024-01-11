use axum::{Router};
use axum::http::StatusCode;
use axum::routing::{get};
use shared::utils::{route};

pub fn health_routes() -> Router {
    route("/health", get(health_check))
}

async fn health_check() -> Result<&'static str, (StatusCode, &'static str)> {
    Ok("OK")
}
