use async_trait::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::{request::Parts};
use serde::Deserialize;
use crate::CustomError;

#[derive(Debug, Clone, Deserialize)]
struct Limit {
  limit: u64,
}

impl Default for Limit {
  fn default() -> Self {
    Self { limit: 100 }
  }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Offset {
  offset: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Page {
  page: u64,
}

#[derive(Debug, Clone)]
pub struct PaginationQuery {
  pub page: u64,
  /// The number of documents to skip before counting
  pub offset: u64,
  /// The maximum number of documents to query
  pub limit: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for PaginationQuery
where
  S: Send + Sync,
{
  type Rejection = CustomError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let Query(Page { page }) = Query::<Page>::from_request_parts(parts, state)
        .await.map_err(|e| CustomError::AxumQueryRejection(e))?;

    let Query(Limit { limit }) = Query::<Limit>::from_request_parts(parts, state)
      .await.map_err(|e| CustomError::AxumQueryRejection(e))?;

    let Query(Offset { offset }) = Query::<Offset>::from_request_parts(parts, state)
      .await
      .unwrap_or_default();

    Ok(Self { page, limit, offset })
  }
}
