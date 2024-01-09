use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request}, Json, RequestPartsExt};
use futures::TryFutureExt;
use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationError, ValidationErrors};
use crate::infra::{CustomError, JsonError};

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
        // let (mut parts, body) = req.into_parts();
        // let req = Request::from_parts(parts, body);
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        // let Json(value) = Json::<T>::from_request(req, state).await?;

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => {
                Ok(Self(value.0))
            },
            Err(rejection) => {
                let payload = JsonError {
                    message: rejection.body_text(),
                    origin: Some("custom_extractor".to_string()),
                    path: path,
                };
                Err(CustomError::AxumJsonRejection(rejection))
            }
        }
    }
}
