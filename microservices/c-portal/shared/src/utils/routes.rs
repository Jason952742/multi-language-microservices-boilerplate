use axum::Router;
use axum::routing::MethodRouter;

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}