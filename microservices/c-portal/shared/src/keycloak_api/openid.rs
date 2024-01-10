use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_derive::{Deserialize, Serialize};
use crate::client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub session_state: String,
    pub scope: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TokenRequestBody {
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshTokenRequestBody {
    pub client_id: String,
    pub grant_type: String,
    pub refresh_token: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientTokenRequestBody {
    pub client_id: String,
    pub grant_type: String,
    pub client_secret: String,
}

pub async fn get_token(path: &str, payload: TokenRequestBody) -> Result<Token, reqwest::Error> {
    let client = client().await;

    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .form(&[
            ("grant_type", payload.grant_type),
            ("client_id", payload.client_id),
            ("client_secret", payload.client_secret.map_or("".to_string(), |x| x)),
            ("username", payload.username),
            ("password", payload.password),
            ("code", payload.code.map_or("".to_string(), |x| x)),
            ("redirect_uri", payload.redirect_uri.map_or("".to_string(), |x| x)),
        ])
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}

pub async fn get_refersh_token(path: &str, payload: RefreshTokenRequestBody) -> Result<Token, reqwest::Error> {
    let client = client().await;
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .form(&[
            ("grant_type", payload.grant_type),
            ("client_id", payload.client_id),
            ("refresh_token", payload.refresh_token),
        ])
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}

pub async fn get_client_token(path: &str, payload: ClientTokenRequestBody) -> Result<Token, reqwest::Error> {
    let client = client().await;
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .form(&[
            ("grant_type", payload.grant_type),
            ("client_id", payload.client_id),
            ("client_secret", payload.client_secret),
        ])
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}

pub async fn introspect_token(path: &str, payload: serde_json::Value) -> Result<String, reqwest::Error> {
    let client = client().await;
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.text().await
}
