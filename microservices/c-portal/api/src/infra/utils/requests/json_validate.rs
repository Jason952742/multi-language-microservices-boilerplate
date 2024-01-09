use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}, Json};
use serde::Deserialize;
use validator::{Validate};
use crate::infra::{AxumJsonRejection, CustomError, JsonError};

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<'a, S, T> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    T: Deserialize<'a> + Validate + std::fmt::Debug,
{
    type Rejection = CustomError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => {
                value.validate()?;
                Ok(Self(value.0))
            },
            Err(rejection) => {
                let payload = JsonError {
                    message: rejection.body_text(),
                    origin: Some("custom_extractor".to_string()),
                };
                Err(CustomError::AxumJsonRejection(AxumJsonRejection(Json(payload))))
            }
        }
    }
}
