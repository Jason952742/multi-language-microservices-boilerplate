
pub mod requests;
pub mod responses;
pub mod errors;

pub use requests::*;
pub use responses::*;
pub use errors::*;

use axum::Router;
use axum::routing::MethodRouter;


pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
