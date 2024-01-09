use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts}
};
use serde::de::DeserializeOwned;
use crate::infra::CustomError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for ValidatedPath<T>
    where
        // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
        T: DeserializeOwned + Send,
        S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                Err(CustomError::AxumPathRejection(rejection))
            }
        }
    }
}