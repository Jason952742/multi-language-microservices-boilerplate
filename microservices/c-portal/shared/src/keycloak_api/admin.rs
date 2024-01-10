use reqwest::header::{HeaderValue, CONTENT_TYPE};
use crate::client;

pub async fn payload_bearer_request(
    path: &str,
    payload: serde_json::Value,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    client().await.post(path)
        .bearer_auth(token.to_string())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&payload)
        .send().await?.error_for_status()
}

pub async fn payload_bearer_request_status(
    path: &str,
    payload: serde_json::Value,
    token: &str,
) -> Result<reqwest::StatusCode, reqwest::Error> {
    client().await.post(path)
        .bearer_auth(token.to_string())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&payload)
        .send().await.map(|response| response.status())
}

pub async fn bearer_post_request(
    path: &str,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    client().await.post(path)
        .bearer_auth(token.to_string())
        .send().await?.error_for_status()
}

pub async fn bearer_get_request(
    path: &str,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    client().await.get(path).bearer_auth(token.to_string()).send().await?.error_for_status()
}
