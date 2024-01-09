use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::infra::{AuthError, CustomError};
use crate::infra::dto::auth::KEYS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
    where
        S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| CustomError::Authenticate(AuthError::MissingToken))?;

        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| CustomError::Authenticate(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}
