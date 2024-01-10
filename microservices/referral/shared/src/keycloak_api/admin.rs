use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::json;
use crate::keycloak_api::urls::AdminUrl;
use crate::keycloak_api::{client, CredentialRepresentation, ExecuteActionsEmailQuery, GroupRepresentation, RoleRepresentation, UserGroupsQuery, UserQuery, UserRepresentation};

pub async fn create_user(base_url: &str, data: &UserRepresentation, realm_name: &str, token: &str) -> Result<Option<String>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUsers { realm_name };
    let payload = serde_json::to_value(data).unwrap();

    let response = client().await
        .post(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&payload)
        .send().await?
        .error_for_status()?;

    if let Some(location) = response.headers().get("location").and_then(|location| location.to_str().ok()) {
        Ok(location.rsplitn(2, '/').next().map(|id| id.to_owned()))
    } else {
        Ok(None)
    }
}

pub async fn update_user(base_url: &str, id: &str, data: &UserRepresentation, realm_name: &str, token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUser { realm_name, id };
    let payload = serde_json::to_value(data).unwrap();

    client().await.put(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&payload)
        .send().await?
        .error_for_status()
        .map(|_| {})
}

pub async fn change_password(base_url: &str, id: &str, data: &CredentialRepresentation, realm_name: &str, token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUserPassword { realm_name, id };
    let payload = serde_json::to_value(data).unwrap();

    client().await
        .put(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&payload)
        .send().await?
        .error_for_status()
        .map(|_| {})
}

pub async fn get_user(base_url: &str, realm_name: &str, id: &str, token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUser { realm_name, id };

    let response = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;
    let json = response.json().await?;

    if let Ok(user) = serde_json::from_value(json) {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub async fn get_user_by_name(base_url: &str, realm_name: &str, username: &str, token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUsername { realm_name, username };

    let response = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;
    let json = response.json().await?;

    let user_reps: Vec<UserRepresentation> = serde_json::from_value(json).unwrap();

    if user_reps.is_empty() {
        Ok(None)
    } else {
        Ok(user_reps.first().map(|x| x.to_owned()))
    }
}

pub async fn get_users(base_url: &str, realm_name: &str, query: &UserQuery, token: &str) -> Result<Vec<UserRepresentation>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUsers { realm_name };

    let response = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
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

pub async fn delete_user(base_url: &str, id: &str, realm_name: &str, token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUser { realm_name, id };

    client().await
        .delete(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;
    Ok(())
}

pub async fn users_count(base_url: &str, realm_name:&str, token: &str) -> Result<Option<u64>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUsersCount { realm_name };

    let res = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;

    if let serde_json::Value::Number(count) = res.json().await? {
        Ok(count.as_u64())
    } else {
        Ok(None)
    }
}

pub async fn add_user_group<'a>(base_url: &'a str, realm_name: &str, id: &str, group_id: &str, token: &'a str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUserGroup { realm_name, id, group_id };

    let k_res = client().await
        .put(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .json(&json!({
                "realm": realm_name.to_owned(),
                "userId": id.to_owned(),
                "groupId": group_id.to_owned(),
            }))
        .send().await?
        .error_for_status()?;
    k_res.text().await?;
    Ok(())
}

pub async fn remove_user_group<'a>(base_url: &'a str, realm_name: &str, id: &str, group_id: &str, token: &'a str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUserGroup { realm_name, id, group_id };

    let k_res = client().await
        .delete(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .json(&json!({
                "realm": realm_name.to_owned(),
                "userId": id.to_owned(),
                "groupId": group_id.to_owned(),
            }))
        .send().await?
        .error_for_status()?;
    k_res.text().await?;
    Ok(())
}

pub async fn user_representation(base_url: &str, realm_name: &str, id: &str, token: &str) -> Result<Option<UserRepresentation>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUser { realm_name, id };

    let k_res = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .send().await?
        .error_for_status()?;
    Ok(serde_json::from_value(k_res.json().await?).ok())
}

pub async fn user_groups(base_url: &str, realm_name: &str, id: &str, query: Option<UserGroupsQuery<'_>>, token: &str) -> Result<Option<Vec<GroupRepresentation>>, reqwest::Error> {
    let url = AdminUrl::UrlAdminUserGroups { realm_name, id };

    let request = client().await
        .get(format!("{base_url}/{url}"))
        .bearer_auth(token);

    let request = if let Some(query) = query {
        request.query(&query)
    } else {
        request
    };
    let k_res = request.send().await?.error_for_status()?;
    Ok(serde_json::from_value(k_res.json().await?).ok())
}

pub async fn add_realm_roles_to_user(base_url: &str, realm_name: &str, id: &str, roles: &[RoleRepresentation], token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUserRealmRoles { realm_name, id };

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .json(roles)
        .send().await?
        .error_for_status()?;
    k_res.text().await?;
    Ok(())
}

pub async fn add_client_roles_to_user(base_url: &str, realm_name: &str, id: &str, client_id: &str, roles: &[RoleRepresentation], token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminUserClientRoles { realm_name, id, client_id };

    let k_res = client().await
        .post(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .json(roles)
        .send().await?
        .error_for_status()?;

    k_res.text().await?;
    Ok(())
}

pub async fn send_update_account(base_url: &str, realm_name: &str, id: &str, actions: &[&str], lifespan: i32, client_id: Option<&str>, redirect_uri: Option<&str>, token: &str) -> Result<(), reqwest::Error> {
    let url = AdminUrl::UrlAdminSendUpdateAccount { realm_name, id };
    let query = ExecuteActionsEmailQuery { lifespan, client_id, redirect_uri };

    client().await
        .put(format!("{base_url}/{url}"))
        .bearer_auth(token)
        .query(&query)
        .json(&actions)
        .send().await?
        .error_for_status()?;

    Ok(())
}
