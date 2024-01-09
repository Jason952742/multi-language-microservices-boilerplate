use axum::{Router};
use axum::http::StatusCode;
use axum::routing::{get};
use crate::infra::{AppState, route};

pub fn health_routes() -> Router {
    route("/health", get(HealthService::health_check))
}

pub struct HealthService;

impl HealthService {
    pub async fn health_check() -> Result<&'static str, (StatusCode, &'static str)> {
        Ok("OK")
    }

}
