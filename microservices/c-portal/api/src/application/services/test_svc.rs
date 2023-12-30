use axum::{Json, Router};
use axum::response::Html;
use axum::routing::{get, post};
use jsonwebtoken::{encode, Header};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::infra::{AppState, AuthBody, AuthError, AuthPayload, Claims, KEYS, route, ValidatedForm, Version};

pub fn test_routes() -> Router<AppState> {
    route(
        "/protected",
        get(TestService::protected),
    ).route(
        "/authorize",
        post(TestService::authorize),
    ).route(
        "/validate",
        get(TestService::validate),
    ).route(
        "/:version/foo",
        get(TestService::version),
    )
}

pub struct TestService;

impl TestService {
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

    async fn version(version: Version) -> Result<Json<User>, AuthError> {
        println!("received request with version {version:?}");
        let user = User::default();
        Ok(Json(user))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct NameInput {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserParams {
    name: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct User {
    id: u64,
    name: String,
    created_at: chrono::NaiveDateTime,
}
