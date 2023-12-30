use axum::{Form, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use crate::infra::{AppState, FlashData, Params, route};

pub fn health_routes() -> Router<AppState> {
    route("/health", get(HealthService::health_check))
}

pub struct HealthService;

impl HealthService {
    pub async fn health_check(state: State<AppState>) -> Result<&'static str, (StatusCode, &'static str)> {
        Ok("OK")
    }

}


