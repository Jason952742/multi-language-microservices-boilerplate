use std::collections::HashMap;
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts},
    RequestPartsExt,
};
use crate::utils::CustomError;


#[derive(Debug)]
pub enum Version {
    V1,
    V2,
    V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
    where
        S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(|_| CustomError::BadVersion("path param missing".to_string()))?;

        let version = params
            .get("version")
            .ok_or_else(|| CustomError::BadVersion("version param missing".to_string()))?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err(CustomError::BadVersion("unknown version".to_string())),
        }
    }
}
