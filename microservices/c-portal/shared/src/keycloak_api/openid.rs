use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::json;
use crate::keycloak_api::{client, ClientTokenRequestBody, RefreshTokenRequestBody, Token, TokenRequestBody};
use crate::keycloak_api::urls::OpenIdUrl;

pub async fn user_info(base_url: &str, realm_name: String, bearer: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = OpenIdUrl::UrlUserinfo { realm_name };
    let path = format!("{base_url}{url}");

    let k_res = client().await.post(&path).bearer_auth(bearer).send().await?.error_for_status()?;
    Ok(json!(k_res.json().await?))
}

pub async fn well_known(base_url: &str, realm_name: String) -> Result<String, reqwest::Error> {
    let url = OpenIdUrl::UrlWellKnown { realm_name }.to_string();
    let path = format!("{base_url}{url}");

    let res = client().await.post(&path).send().await?;
    res.text().await
}

pub async fn token(base_url: &str, data: TokenRequestBody, realm_name: String) -> Result<Token, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();
    let path = format!("{base_url}{url}");

    get_token(&path, data).await
}

pub async fn token_client(base_url: &str, realm_name: String, client_id: &str, client_secret: &str) -> Result<Token, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();
    let path = format!("{base_url}{url}");

    let payload = ClientTokenRequestBody {
        client_id: client_id.to_owned(),
        grant_type: client_secret.to_owned(),
        client_secret: "client_credentials".to_owned(),
    };

    get_client_token(&path, payload).await
}

pub async fn introspect(base_url: &str, realm_name: String, data: serde_json::Value) -> Result<String, reqwest::Error> {
    let url = OpenIdUrl::UrlIntrospect { realm_name }.to_string();
    let path = format!("{base_url}{url}");

    let payload = json!({
            "client_id":data["client_id"],
            "client_secret":data["client_secret"],
            "token":data["token"],
        });

    introspect_token(&path, payload).await
}

pub async fn refresh_token(base_url: &str, data: RefreshTokenRequestBody, realm_name: String) -> Result<String, reqwest::Error> {
    let url = OpenIdUrl::UrlToken { realm_name }.to_string();
    let path = format!("{base_url}{url}");

    let res = get_refersh_token(&path, data).await?;
    let d = json!(res);
    let token = d["access_token"].to_string();
    Ok(token)
}

async fn get_token(path: &str, payload: TokenRequestBody) -> Result<Token, reqwest::Error> {
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

async fn get_refersh_token(path: &str, payload: RefreshTokenRequestBody) -> Result<Token, reqwest::Error> {
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

async fn get_client_token(path: &str, payload: ClientTokenRequestBody) -> Result<Token, reqwest::Error> {
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

async fn introspect_token(path: &str, payload: serde_json::Value) -> Result<String, reqwest::Error> {
    let client = client().await;
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.text().await
}
