use reqwest::header::{HeaderValue, CONTENT_TYPE};
use crate::{admin, client};
use crate::openid;
use jsonwebtoken::{decode_header, errors::Error as JwtError};
use serde_json::json;
use tonic::codegen::Body;
use crate::keycloak_api::model::{ExecuteActionsEmailQuery, GroupRepresentation, RoleRepresentation, UserGroupsQuery, UserQuery, UserRepresentation};
use crate::openid::{ClientTokenRequestBody, RefreshTokenRequestBody, Token, TokenRequestBody};
use crate::urls::{AdminUrl, OpenIdUrl};

pub struct Admin();

pub struct OpenId();

impl OpenId {
    pub async fn well_known(base_url: &str, realm_name: String) -> Result<String, reqwest::Error> {
        let url = OpenIdUrl::UrlWellKnown { realm_name }.to_string();
        let path = format!("{base_url}{url}");

        let res = client().await.post(&path).send().await?;
        res.text().await
    }

    pub async fn token(base_url: &str, data: TokenRequestBody, realm_name: String) -> Result<Token, reqwest::Error> {
        let url = OpenIdUrl::UrlToken { realm_name }.to_string();
        let path = format!("{base_url}{url}");

        openid::get_token(&path, data).await
    }

    pub async fn token_client(base_url: &str, realm_name: String, client_id: &str, client_secret: &str) -> Result<Token, reqwest::Error> {
        let url = OpenIdUrl::UrlToken { realm_name }.to_string();
        let path = format!("{base_url}{url}");

        let payload = ClientTokenRequestBody {
            client_id: client_id.to_owned(),
            grant_type: client_secret.to_owned(),
            client_secret: "client_credentials".to_owned(),
        };

        openid::get_client_token(&path, payload).await
    }

    pub async fn introspect(base_url: &str, realm_name: String, data: serde_json::Value) -> Result<String, reqwest::Error> {
        let url = OpenIdUrl::UrlIntrospect { realm_name }.to_string();
        let path = format!("{base_url}{url}");

        let payload = json!({
            "client_id":data["client_id"],
            "client_secret":data["client_secret"],
            "token":data["token"],
        });

        openid::introspect_token(&path, payload).await
    }

    pub fn jwt_decode(token: String) -> Result<jsonwebtoken::Header, JwtError> {
        decode_header(&token)
    }

    pub async fn refresh_token(base_url: &str, data: RefreshTokenRequestBody, realm_name: String) -> Result<String, reqwest::Error> {
        let url = OpenIdUrl::UrlToken { realm_name }.to_string();
        let path = format!("{base_url}{url}");

        let res = openid::get_refersh_token(&path, data).await?;
        let d = json!(res);
        let token = d["access_token"].to_string();
        Ok(token)
    }
}

impl Admin {
    pub async fn create_user(base_url: &str, data: &UserRepresentation, realm_name: String, token: &str) -> Result<Option<String>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUsers { realm_name };
        let path = format!("{base_url}{url}");

        let payload = serde_json::to_value(data).unwrap();
        let response = admin::payload_bearer_request(&path, payload, token).await?;

        if let Some(location) = response.headers().get("location").and_then(|location| location.to_str().ok()) {
            Ok(location.rsplitn(2, '/').next().map(|id| id.to_owned()))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user(base_url: &str, data: &UserRepresentation, realm_name: String, token: &str) -> Result<(), reqwest::Error> {
        let id = data.clone().id.unwrap();
        let url = AdminUrl::UrlAdminUser { realm_name, id };
        let path = format!("{base_url}{url}");

        let payload = serde_json::to_value(data).unwrap();
        client().await.put(&path)
            .bearer_auth(token.to_string())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .json(&payload)
            .send().await?.error_for_status()
            .map(|_| {})
    }

    pub async fn get_user(base_url: &str, realm_name: String, user_id: String, token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUser { realm_name, id: user_id };
        let path = format!("{base_url}{url}");

        let response = client().await.get(&path)
            .bearer_auth(token.to_string())
            .send().await?.error_for_status()?;
        let json = response.json().await?;

        if let Ok(user) = serde_json::from_value(json) {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_name(base_url: &str, realm_name: String, username: String, token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUsername { realm_name, username };
        let path = format!("{base_url}{url}");

        let response = client().await.get(&path)
            .bearer_auth(token.to_string())
            .send().await?.error_for_status()?;
        let json = response.json().await?;

        let user_reps: Vec<UserRepresentation> = serde_json::from_value(json).unwrap();

        if user_reps.is_empty() {
            Ok(None)
        } else {
            Ok(user_reps.first().map(|x| x.to_owned()))
        }
    }

    pub async fn get_users(base_url: &str, realm_name: String, query: &UserQuery, token: &str) -> Result<Vec<UserRepresentation>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUsers { realm_name };
        let path = format!("{base_url}{url}");

        let response = client().await
            .get(&path)
            .bearer_auth(token.to_string())
            .query(&query)
            .send()
            .await?.error_for_status()?;
        let json = response.json().await?;

        if let Ok(users) = serde_json::from_value(json) {
            Ok(users)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn delete_user(base_url: &str, user_id: String, realm_name: String, token: &str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminUser { realm_name, id: user_id };
        let path = format!("{base_url}{url}");

        client().await.delete(&path)
            .bearer_auth(token.to_string())
            .send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn users_count(base_url: &str, realm_name: String, bearer: &str) -> Result<Option<u64>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUsersCount { realm_name };
        let path = format!("{base_url}{url}");

        let res = admin::bearer_get_request(&path, bearer).await?;
        if let serde_json::Value::Number(count) = res.json().await? {
            Ok(count.as_u64())
        } else {
            Ok(None)
        }
    }

    pub async fn user_info(base_url: &str, realm_name: String, bearer: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url = OpenIdUrl::UrlUserinfo { realm_name };
        let path = format!("{base_url}{url}");

        let k_res = client().await.post(&path).bearer_auth(bearer).send().await?.error_for_status()?;
        Ok(json!(k_res.json().await?))
    }

    pub async fn add_user_group<'a>(base_url: &'a str, realm_name: String, user_id: String, group_id: String, bearer: &'a str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminUserGroup { realm_name: realm_name.clone(), id: user_id.clone(), group_id: group_id.clone() };
        let path = format!("{base_url}{url}");

        let k_res = client().await.put(&path).bearer_auth(bearer)
            .json(&json!({
                "realm": realm_name.to_owned(),
                "userId": user_id.to_owned(),
                "groupId": group_id.to_owned(),
            }))
            .send().await?.error_for_status()?;
        k_res.text().await?;
        Ok(())
    }

    pub async fn remove_user_group<'a>(base_url: &'a str, realm_name: String, user_id: String, group_id: String, bearer: &'a str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminUserGroup { realm_name: realm_name.clone(), id: user_id.clone(), group_id: group_id.clone() };
        let path = format!("{base_url}{url}");

        let k_res = client().await.delete(&path).bearer_auth(bearer)
            .json(&json!({
                "realm": realm_name.to_owned(),
                "userId": user_id.to_owned(),
                "groupId": group_id.to_owned(),
            }))
            .send().await?.error_for_status()?;
        k_res.text().await?;
        Ok(())
    }

    pub async fn user_representation(base_url: &str, realm_name: String, id: String, bearer: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUser { realm_name, id };
        let path = format!("{base_url}{url}");

        let k_res = client().await.get(&path).bearer_auth(bearer).send().await?.error_for_status()?;
        Ok(serde_json::from_value(k_res.json().await?).ok())
    }

    pub async fn user_groups(base_url: &str, realm_name: String, id: String, query: Option<UserGroupsQuery<'_>>, bearer: &str) -> Result<Option<Vec<GroupRepresentation>>, reqwest::Error> {
        let url = AdminUrl::UrlAdminUserGroups { realm_name, id };
        let path = format!("{base_url}{url}");

        let request = client().await.get(&path).bearer_auth(bearer);
        let request = if let Some(query) = query {
            request.query(&query)
        } else {
            request
        };
        let k_res = request.send().await?.error_for_status()?;
        Ok(serde_json::from_value(k_res.json().await?).ok())
    }

    pub async fn add_realm_roles_to_user(base_url: &str, realm_name: String, user_id: String, roles: &[RoleRepresentation], bearer: &str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminUserRealmRoles { realm_name, id: user_id };
        let path = format!("{base_url}{url}");

        let k_res = client().await.post(&path).bearer_auth(bearer)
            .json(roles)
            .send().await?.error_for_status()?;
        k_res.text().await?;
        Ok(())
    }

    pub async fn add_client_roles_to_user(base_url: &str, realm_name: String, user_id: String, client_id: String, roles: &[RoleRepresentation], bearer: &str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminUserClientRoles { realm_name, id: user_id, client_id };
        let path = format!("{base_url}{url}");

        let k_res = client().await.post(&path).bearer_auth(bearer)
            .json(roles)
            .send().await?.error_for_status()?;
        k_res.text().await?;
        Ok(())
    }

    pub async fn send_update_account(base_url: &str, realm_name: String, user_id: String, actions: &[&str], lifespan: i32, client_id: Option<&str>, redirect_uri: Option<&str>, bearer: &str) -> Result<(), reqwest::Error> {
        let url = AdminUrl::UrlAdminSendUpdateAccount { realm_name, id: user_id };
        let path = format!("{base_url}{url}");

        let query = ExecuteActionsEmailQuery {
            lifespan,
            client_id,
            redirect_uri,
        };

        client().await.put(&path).bearer_auth(bearer)
            .query(&query)
            .json(&actions)
            .send().await?.error_for_status()?;
        Ok(())
    }
}
