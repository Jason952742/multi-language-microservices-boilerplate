use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct UserConsentRepresentation {
    pub client_id: Option<String>,
    pub created_date: Option<i64>,
    pub granted_client_scopes: Option<Vec<String>>,
    pub last_update_date: Option<i64>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct CredentialRepresentation {
    pub algorithm: Option<String>,
    pub config: serde_json::Value,
    pub counter: Option<i32>,
    pub created_date: Option<i64>,
    pub device: Option<String>,
    pub digits: Option<i32>,
    pub hash_iterations: Option<i32>,
    pub hashed_salted_value: Option<String>,
    pub period: Option<i32>,
    pub salt: Option<String>,
    pub temporary: Option<bool>,
    pub r#type: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub struct FederatedIdentityRepresentation {
    pub identity_provider: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all="camelCase")]
pub struct UserRepresentation {
    pub access: Option<HashMap<String, bool>>,
    pub attributes: Option<HashMap<String, Vec<String>>>,
    pub client_consents: Option<Vec<UserConsentRepresentation>>,
    pub created_timestamp: Option<i64>,
    pub credentials: Option<Vec<CredentialRepresentation>>,
    pub disableable_credential_types: Option<Vec<String>>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub enabled: Option<bool>,
    pub federated_identities: Option<Vec<FederatedIdentityRepresentation>>,
    pub federation_link: Option<String>,
    pub first_name: Option<String>,
    pub groups: Option<Vec<String>>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub not_before: Option<i32>,
    pub origin: Option<String>,
    pub realm_roles: Option<Vec<String>>,
    pub required_actions: Option<Vec<String>>,
    #[serde(rename="self")]
    pub self_: Option<String>,
    pub service_account_client_id: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RoleRepresentationComposites {
    pub client: Option<HashMap<String, String>>,
    pub realm: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all="camelCase")]
pub struct RoleRepresentation {
    pub attributes: Option<HashMap<String, String>>,
    pub client_role: Option<bool>,
    pub composite: Option<bool>,
    pub composites: Option<RoleRepresentationComposites>,
    pub container_id: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl RoleRepresentation {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: Some(id.to_owned()),
            name: Some(name.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all="camelCase")]
pub struct UserQuery {
    pub brief_representation: Option<bool>,
    pub email: Option<String>,
    pub first: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub max: Option<i32>,
    pub search: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExecuteActionsEmailQuery<'a> {
    pub lifespan: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserGroupsQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GroupRepresentation {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_expires_in: i64,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserClaim {
    pub sub: String,
    pub email_verified: bool,
    pub preferred_username: String
}