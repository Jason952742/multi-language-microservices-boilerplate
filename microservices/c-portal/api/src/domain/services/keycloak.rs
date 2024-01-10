use std::collections::HashMap;
use chrono::Utc;
use std::env;
use shared::keycloak_api;
use shared::keycloak_api::model::CredentialRepresentation;
use shared::keycloak_api::{Token, TokenRequestBody, UserRepresentation};
use once_cell::sync::Lazy;

static BASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("KEYCLOAK_HOST").expect("KEYCLOAK_HOST must be set")
});

pub async fn get_admin_token() -> Result<Token, reqwest::Error> {
    dotenvy::dotenv().ok();
    let payload = TokenRequestBody {
        username: env::var("KEYCLOAK_ADMIN_USER").expect("KEYCLOAK_ADMIN_USER must be set"),
        password: env::var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set"),
        client_id: env::var("KEYCLOAK_ADMIN_CLIENT").expect("KEYCLOAK_ADMIN_CLIENT must be set"),
        grant_type: "password".to_string(),
        scope: Some("openid".to_string()),
        ..Default::default()
    };
    let access_token = keycloak_api::openid::password_token(&**BASE_URL, payload, "master").await?;
    Ok(access_token)
}

pub async fn get_user_token(username: &str, password: &str) -> Result<Token, reqwest::Error> {
    dotenvy::dotenv().ok();
    let payload = TokenRequestBody {
        username: username.to_string(),
        password: password.to_string(),
        client_id: env::var("KEYCLOAK_USER_CLIENT").expect("KEYCLOAK_USER_CLIENT must be set"),
        client_secret: Some(env::var("KEYCLOAK_USER_CLIENT_SECRET").expect("KEYCLOAK_USER_CLIENT_SECRET must be set")),
        grant_type: "password".to_string(),
        scope: Some("openid".to_string()),
        ..Default::default()
    };
    let realm = env::var("KEYCLOAK_USER_REALM").expect("KEYCLOAK_USER_REALM must be set");
    let access_token = keycloak_api::openid::password_token(&**BASE_URL, payload, &realm).await?;
    Ok(access_token)
}

pub async fn get_user(username: &str, admin_token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
    let realm = env::var("KEYCLOAK_USER_REALM").expect("KEYCLOAK_USER_REALM must be set");
    let user = keycloak_api::admin::get_user_by_name(&**BASE_URL, &realm, username, admin_token).await?;
    Ok(user)
}

pub async fn create_user(username: &str, password: &str, admin_token: &str) -> Result<Option<String>, reqwest::Error> {
    let mut attributes: HashMap<String, Vec<String>> = HashMap::new();
    attributes.insert("expiredAt".to_string(), vec![Utc::now().to_string()]);
    let credentials: Vec<CredentialRepresentation> = vec![
        CredentialRepresentation {
            r#type: Some("password".to_string()),
            value: Some(password.to_string()),
            temporary: Some(false),
            ..Default::default()
        }
    ];

    let user = UserRepresentation {
        username: Some(username.to_string()),
        enabled: Some(true),
        attributes: Some(attributes),
        credentials: Some(credentials),
        ..Default::default()
    };

    let realm = env::var("KEYCLOAK_USER_REALM").expect("KEYCLOAK_USER_REALM must be set");
    let result = keycloak_api::admin::create_user(&**BASE_URL, &user, &realm, admin_token).await?;

    Ok(result)
}

pub async fn change_password(id: String, new_password: String, admin_token: &str) -> Result<(), reqwest::Error> {
    let credential = CredentialRepresentation {
        r#type: Some("password".to_string()),
        value: Some(new_password),
        temporary: Some(false),
        ..Default::default()
    };

    let realm = env::var("KEYCLOAK_USER_REALM").expect("KEYCLOAK_USER_REALM must be set");
    keycloak_api::admin::change_password(&**BASE_URL, &*id, &credential, &realm, admin_token).await?;

    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = get_admin_token().await?;

    change_password("4cc2477a-b2a4-428e-817e-66c561e7237d".to_string(), "456".to_string(), &token.access_token).await?;

    // println!("{:?}", user);

    Ok(())
}


