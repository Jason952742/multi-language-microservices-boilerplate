use std::env;
use std::fmt::Display;
use async_trait::async_trait;
use sea_orm_migration::MigratorTrait;
use tera::Tera;
use colored::Colorize;
use tokio::net::TcpListener;
use shared::datasource::postgres::PgPool;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;
use crate::infra::migration::Migrator;
use crate::application::services::{health_routes, post_routes};
use crate::infra::AppState;
use listenfd::ListenFd;
use shared::consul_api;
use axum::{http::StatusCode, routing::{get_service}, Router, Json, RequestPartsExt};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

mod flash;
mod infra;
mod domain;
mod application;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // set log level
    env::set_var("RUST_LOG", "debug");

    // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
    // will be written to stdout.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_test_writer()
        .init();

    // load evn
    dotenvy::dotenv().ok();

    // establish database connection
    let connection = PgPool::conn().await.clone();
    Migrator::up(&connection, None).await?;

    // get host addr
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let server_url = format!("{host}:{port}");

    // register consul service
    consul_register(&host, port.parse().unwrap()).await;

    // establish database connection
    let conn = PgPool::conn().await.clone();
    Migrator::up(&conn, None).await?;

    // create app state
    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");
    let state: AppState = AppState { templates, conn };

    let app = Router::new()
        .merge(post_routes())
        .merge(health_routes())
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
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
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => TcpListener::from_std(listener).unwrap(),
        // otherwise fall back to local listening
        None => TcpListener::bind(&server_url).await.unwrap(),
    };

    // run it
    println!("listening on {}", &server_url.color("magenta"));
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

// register consul service
async fn consul_register(host: &str, port: i32) {
    let cs = consul_api::Consul::new(consul_api::ConsulOption::default()).unwrap();
    let reg = consul_api::Registration::simple(consul_api::ServiceName::MuCPortal, host, port, false);
    cs.register(&reg).await.unwrap();
    tokio::spawn(async move {
        cs.discover_service().await.expect("discover_service failed");
    });
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "hello@world.io".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}