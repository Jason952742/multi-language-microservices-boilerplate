use std::env;
use tera::Tera;
use colored::Colorize;
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;
use crate::application::rest::{health_routes, test_routes, post_routes};
use crate::infra::AppState;
use listenfd::ListenFd;
use shared::Config;
use axum::{http::StatusCode, routing::{get_service}, Router};
use shared::mongodb::Client;

mod flash;
mod infra;
mod domain;
mod application;

/// API entry
///
pub async fn start(config: Config, conn: Client) -> anyhow::Result<()> {
    // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
    // will be written to stdout.


    // create app state
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state: AppState = AppState { templates, conn };

    let app = api_router()
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let server_url = format!("{}:{}", config.host, config.port);

    // listen addr
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => TcpListener::from_std(listener).unwrap(),
        // otherwise fall back to local listening
        None => TcpListener::bind(&server_url).await.unwrap(),
    };

    // run it
    tracing::info!("C-PortalServer listening on {}", &server_url.color("magenta"));
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// App Router
///
fn api_router() -> Router<AppState> {
    Router::new()
        .merge(post_routes())
        .merge(health_routes())
        .merge(test_routes())
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
                .handle_error(|error| async move {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}"))
                }),
        )
}
