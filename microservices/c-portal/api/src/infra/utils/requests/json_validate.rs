use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request},
    RequestPartsExt,
};
use serde_json::{json, Value};
use crate::infra::CustomError;

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                Err(CustomError::JsonValidationError(rejection.body_text()))
            }
        }
    }
}
