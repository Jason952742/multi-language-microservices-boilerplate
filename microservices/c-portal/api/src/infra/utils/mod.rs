pub mod jwt;
pub mod requests;
pub mod responses;
pub mod errors;

pub use jwt::*;
pub use requests::*;
pub use responses::*;
pub use errors::*;

use axum::Router;
use axum::routing::MethodRouter;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct PageingParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
