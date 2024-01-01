pub mod jwt_util;
pub mod request_util;
pub mod response_util;
pub mod errors_util;
pub mod path_utils;

pub use jwt_util::*;
pub use request_util::*;
pub use response_util::*;
pub use errors_util::*;
pub use path_utils::*;

use axum::Router;
use axum::routing::MethodRouter;
use serde_derive::{Deserialize, Serialize};
use tera::Tera;
use shared::mongodb::Client;

#[derive(Clone)]
pub struct AppState {
    pub templates: Tera,
    pub conn: Client,
}

#[derive(Deserialize)]
pub struct Params {
    pub page: Option<u32>,
    pub posts_per_page: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub kind: String,
    pub message: String,
}

pub fn route(path: &str, method_router: MethodRouter<AppState>) -> Router<AppState> {
    Router::new().route(path, method_router)
}
