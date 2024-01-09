use async_trait::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::{request::Parts, StatusCode};
use serde::Deserialize;

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
pub struct Pagination {
  pub page: u64,
  /// The number of documents to skip before counting
  pub offset: u64,
  /// The maximum number of documents to query
  pub limit: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
  S: Send + Sync,
{
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let Query(Page { page }) = Query::<Page>::from_request_parts(parts, state)
        .await
        .unwrap_or_default();

    let Query(Limit { limit }) = Query::<Limit>::from_request_parts(parts, state)
      .await
      .unwrap_or_default();

    let Query(Offset { offset }) = Query::<Offset>::from_request_parts(parts, state)
      .await
      .unwrap_or_default();

    Ok(Self { page, limit, offset })
  }
}
