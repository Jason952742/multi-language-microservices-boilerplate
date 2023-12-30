use axum::{Json, Router};
use axum::response::Html;
use axum::routing::{get, post};
use jsonwebtoken::{encode, Header};
use serde_derive::Deserialize;
use validator::Validate;
use crate::infra::{AppState, AuthBody, AuthError, AuthPayload, Claims, KEYS, route, ValidatedForm, Version};

pub fn jwttest_routes() -> Router<AppState> {
    route(
        "/protected",
        get(JwtTestService::protected),
    ).route(
        "/authorize",
        post(JwtTestService::authorize),
    ).route(
        "/validate",
        get(JwtTestService::validate),
    ).route(
        "/:version/foo",
        get(JwtTestService::version),
    )
}

pub struct JwtTestService;

impl JwtTestService {
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

    async fn protected(claims: Claims) -> Result<String, AuthError> {
        // Send the protected data to the user
        Ok(format!(
            "Welcome to the protected area :)\nYour data:\n{claims}",
        ))
    }

    async fn validate(_claims: Claims, ValidatedForm(input): ValidatedForm<NameInput>) -> Html<String> {
        Html(format!("<h1>Hello, {}!</h1>", input.name))
    }

    async fn version(version: Version) {
        println!("received request with version {version:?}");
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct NameInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}
