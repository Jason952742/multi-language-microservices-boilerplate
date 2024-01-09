use axum::{Json, Router};
use axum::routing::{get, post};
use jsonwebtoken::{encode, Header};
use crate::infra::{Claims, CustomError, AuthError};
use crate::infra::dto::auth::{AuthBody, AuthPayload, KEYS};

pub fn jwt_routes() -> Router {
    Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, CustomError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(CustomError::Authenticate(AuthError::MissingCredentials));
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(CustomError::Authenticate(AuthError::WrongCredentials));
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| CustomError::Authenticate(AuthError::TokenCreation))?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

async fn protected(claims: Claims) -> Result<String, CustomError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}
