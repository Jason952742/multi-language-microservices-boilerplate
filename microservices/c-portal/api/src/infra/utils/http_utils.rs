use axum::Router;
use axum::routing::MethodRouter;
use sea_orm::DatabaseConnection;
use serde_derive::{Deserialize, Serialize};
use tera::Tera;

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
