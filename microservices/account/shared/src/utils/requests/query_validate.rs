use async_trait::async_trait;
use axum::extract::{FromRequest, Request, Query};
use axum::extract::rejection::{QueryRejection};

use serde::de::DeserializeOwned;
use validator::Validate;
use crate::utils::CustomError;


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedQuery<T>
    where
        T: DeserializeOwned + Send + Validate,
        S: Send + Sync,
        Query<T>: FromRequest<S, Rejection=QueryRejection>,
{
    type Rejection = CustomError;

    async fn from_request(req: Request,  state: &S) -> Result<Self, Self::Rejection> {
        match Query::<T>::from_request(req, state).await {
            Ok(value) => {
                value.validate()?;
                Ok(Self(value.0))
            }
            Err(rejection) => {
                Err(CustomError::AxumQueryRejection(rejection))
            }
        }
    }
}
