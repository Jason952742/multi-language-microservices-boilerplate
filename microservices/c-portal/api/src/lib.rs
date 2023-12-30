use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use std::env;
use axum::routing::MethodRouter;
use sea_orm_migration::MigratorTrait;
use tera::Tera;
use shared::datasource::postgres::PgPool;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;
use crate::infra::migration::Migrator;
use crate::application::services::PostService;
use crate::domain::AppState;

mod flash;
mod infra;
mod domain;
mod application;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // set log level
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    // load evn
    dotenvy::dotenv().ok();
    // establish database connection
    let connection = PgPool::conn().await.clone();
    Migrator::up(&connection, None).await?;
    // get host addr
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let server_url = format!("{host}:{port}");
    // establish database connection
    let conn = PgPool::conn().await.clone();
    Migrator::up(&conn, None).await?;

    // create app state
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let state: AppState = AppState { templates, conn };

    let app = Router::new()
        .merge(post_foo()).merge(post_id()).merge(post_new()).merge(post_delete())
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
                .handle_error(|error| async move {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}"))
                }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    // listen addr
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

fn post_foo() -> Router<AppState> {
    route("/", get(PostService::list_posts).post(PostService::create_post))
}

fn post_id() -> Router<AppState> {
    route("/:id", get(PostService::edit_post).post(PostService::update_post))
}

fn post_new() -> Router<AppState> {
    route("/new", get(PostService::new_post))
}

fn post_delete() -> Router<AppState> {
    route("/delete/:id", post(PostService::delete_post))
}

fn route(path: &str, method_router: MethodRouter<AppState>) -> Router<AppState> {
    Router::new().route(path, method_router)
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
