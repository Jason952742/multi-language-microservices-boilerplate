use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::json;
use crate::keycloak_api::{client, ClientTokenRequestBody, RefreshTokenRequestBody, Token, TokenRequestBody, UserClaim};
use crate::keycloak_api::urls::OpenIdUrl;

pub async fn user_info(base_url: &str, realm_name: &str, token: &str) -> Result<UserClaim, reqwest::Error> {
    let url = OpenIdUrl::UrlUserinfo { realm_name };
    let k_res = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;
    k_res.json().await
}

pub async fn well_known(base_url: &str, realm_name: &str) -> Result<String, reqwest::Error> {
    let url = OpenIdUrl::UrlWellKnown { realm_name }.to_string();

    let res = client().await
        .post(format!("{base_url}/{url}"))
        .send().await?;
    res.text().await
}

pub async fn password_token(base_url: &str, payload: TokenRequestBody, realm_name: &str) -> Result<Token, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .form(&[
            ("grant_type", payload.grant_type),
            ("client_id", payload.client_id),
            ("client_secret", payload.client_secret),
            ("username", payload.username),
            ("password", payload.password),
            ("scope", payload.scope),
        ])
        .send().await?
        .error_for_status()?;
    k_res.json().await
}

pub async fn client_token(base_url: &str, realm_name: &str, client_id: &str, client_secret: &str) -> Result<Token, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();
    let payload = ClientTokenRequestBody {
        client_id: client_id.to_owned(),
        grant_type: client_secret.to_owned(),
        client_secret: "client_credentials".to_owned(),
    };

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
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

pub async fn refresh_token(base_url: &str, payload: RefreshTokenRequestBody, realm_name:&str) -> Result<Token, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .form(&[
            ("grant_type", payload.grant_type),
            ("client_id", payload.client_id),
            ("client_secret", payload.client_secret),
            ("refresh_token", payload.refresh_token),
            ("scope", payload.scope),
        ])
        .send()
        .await?.error_for_status()?;

    k_res.json().await
}

pub async fn introspect(base_url: &str, realm_name: &str, data: serde_json::Value) -> Result<String, reqwest::Error> {
    let url = OpenIdUrl::UrlIntrospect { realm_name }.to_string();
    let payload = json!({
            "client_id":data["client_id"],
            "client_secret":data["client_secret"],
            "token":data["token"],
        });

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.text().await
}
