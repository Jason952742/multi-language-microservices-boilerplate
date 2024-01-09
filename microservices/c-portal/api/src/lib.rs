use std::env;
use colored::Colorize;
use tokio::net::TcpListener;
use axum::http::header;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;
use crate::application::rest::{health_routes, test_routes, settings_routes, jwt_routes};
use listenfd::ListenFd;
use shared::Config;
use axum::{http::StatusCode, routing::{get_service}, Router};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, };


mod infra;
mod domain;
mod application;

/// API entry
///
pub async fn start(config: Config) -> anyhow::Result<()> {
    let app = api_router()
        .layer(CookieManagerLayer::new());

    // listen addr
    let mut listenfd = ListenFd::from_env();
    let server_url = format!("{}:{}", config.host, config.port);
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
fn api_router() -> Router {
    Router::new()
        .merge(health_routes())
        .merge(test_routes())
        .merge(Router::new().nest(
            "/api/v1",
            // All public v1 routes will be nested here.
            Router::new()
                .merge(settings_routes())
                .merge(jwt_routes()),
        ))
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
                .handle_error(|error| async move {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}"))
                }),
        )
        // High level logging of requests and responses
        //.layer(
        // trace::TraceLayer::new_for_http()
        //     .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
        // .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
        // .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        //)
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        // CORS configuration. This should probably be more restrictive in production.
        .layer(CorsLayer::permissive())
}
