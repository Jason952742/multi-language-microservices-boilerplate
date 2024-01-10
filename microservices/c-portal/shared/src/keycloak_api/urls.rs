use std::fmt::Display;
use std::string::ToString;

#[derive(Debug)]
pub enum OpenIdUrl {
    UrlWellKnown { realm_name: String },
    UrlToken { realm_name: String },
    UrlUserinfo { realm_name: String },
    UrlIntrospect { realm_name: String },
    UrlLogout { realm_name: String },
    UrlCerts { realm_name: String },
    UrlEntitlement { realm_name: String },
    UrlAuth { endpoint: String, client_id: String, redirect_uri: String },
}

impl Display for OpenIdUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OpenIdUrl::UrlWellKnown { realm_name } => format!("realms/{realm_name}/.well-known/openid-configuration"),
            OpenIdUrl::UrlToken { realm_name } => format!("realms/{realm_name}/protocol/openid-connect/token"),
            OpenIdUrl::UrlUserinfo { realm_name } => format!("realms/{realm_name}/protocol/openid-connect/userinfo"),
            OpenIdUrl::UrlIntrospect { realm_name } => format!("realms/{realm_name}/protocol/openid-connect/token/introspect"),
            OpenIdUrl::UrlLogout { realm_name } => format!("realms/{realm_name}/protocol/openid-connect/logout"),
            OpenIdUrl::UrlCerts { realm_name } => format!("realms/{realm_name}/protocol/openid-connect/certs"),
            OpenIdUrl::UrlEntitlement { realm_name } => format!("realms/{realm_name}/.well-known/openid-configuration"),
            OpenIdUrl::UrlAuth { endpoint, client_id, redirect_uri } => format!("{endpoint}?client_id={client_id}&response_type=code&redirect_uri={redirect_uri}"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub enum AdminUrl {
    UrlAdminUsers { realm_name: String },
    UrlAdminUsersCount { realm_name: String },
    UrlAdminUser { realm_name: String, id: String },
    UrlAdminUsername { realm_name: String, username: String },
    UrlAdminUserConsents { realm_name: String, id: String },
    UrlAdminSendUpdateAccount { realm_name: String, id: String },
    UrlAdminSendVerifyEmail { realm_name: String, id: String },
    UrlAdminResetPassword { realm_name: String, id: String },
    UrlAdminGetSessions { realm_name: String, id: String },
    UrlAdminUserClientRoles { realm_name: String, id: String, client_id: String },
    UrlAdminUserClientRolesAvailable { realm_name: String, id: String, client_id: String },
    UrlAdminUserClientRolesComposite { realm_name: String, id: String, client_id: String },
    UrlAdminUserRealmRoles { realm_name: String, id: String },
    UrlAdminUserGroup { realm_name: String, id: String, group_id: String },
    UrlAdminUserGroups { realm_name: String, id: String },
    UrlAdminUserPassword { realm_name: String, id: String },
    UrlAdminUserStorage { realm_name: String, id: String },
    UrlAdminServerInfo {},
    UrlAdminGroups { realm_name: String },
    UrlAdminGroup { realm_name: String, id: String },
    UrlAdminGroupChild { realm_name: String, id: String },
    UrlAdminGroupPermissions { realm_name: String, id: String },
    UrlAdminGroupMembers { realm_name: String, id: String },
    UrlAdminClients { realm_name: String },
    UrlAdminClient { realm_name: String, id: String },
    UrlAdminClientRoles { realm_name: String, id: String },
    UrlAdminClientRole { realm_name: String, id: String, role_name: String },
    UrlAdminClientAuthzSettings { realm_name: String, id: String },
    UrlAdminClientAuthzResources { realm_name: String, id: String },
    UrlAdminClientCerts { realm_name: String, id: String, attr: String },
    UrlAdminRealmRoles { realm_name: String },
    UrlAdminRealmImport {},
    UrlAdminIdps { realm_name: String },
    UrlAdminFlows { realm_name: String },
    UrlAdminFlowsExecutions { realm_name: String, flow_alias: String },
}

impl Display for AdminUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AdminUrl::UrlAdminUsers { realm_name } => format!("admin/realms/{realm_name}/users"),
            AdminUrl::UrlAdminUsersCount { realm_name } => format!("admin/realms/{realm_name}/users/count"),
            AdminUrl::UrlAdminUser { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}"),
            AdminUrl::UrlAdminUsername { realm_name, username } => format!("admin/realms/{realm_name}/users?username={username}"),
            AdminUrl::UrlAdminUserConsents { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/consents"),
            AdminUrl::UrlAdminSendUpdateAccount { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/execute-actions-email"),
            AdminUrl::UrlAdminSendVerifyEmail { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/send-verify-email"),
            AdminUrl::UrlAdminResetPassword { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/reset-password"),
            AdminUrl::UrlAdminGetSessions { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/sessions"),
            AdminUrl::UrlAdminUserClientRoles { realm_name, id, client_id } => format!("admin/realms/{realm_name}/users/{id}/role-mappings/clients/{client_id}"),
            AdminUrl::UrlAdminUserClientRolesAvailable { realm_name, id, client_id } => format!("admin/realms/{realm_name}/users/{id}/role-mappings/clients/{client_id}/available"),
            AdminUrl::UrlAdminUserClientRolesComposite { realm_name, id, client_id } => format!("admin/realms/{realm_name}/users/{id}/role-mappings/clients/{client_id}/composite"),
            AdminUrl::UrlAdminUserRealmRoles { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/role-mappings/realm"),
            AdminUrl::UrlAdminUserGroup { realm_name, id, group_id } => format!("admin/realms/{realm_name}/users/{id}/groups/{group_id}"),
            AdminUrl::UrlAdminUserGroups { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/groups"),
            AdminUrl::UrlAdminUserPassword { realm_name, id } => format!("admin/realms/{realm_name}/users/{id}/reset-password"),
            AdminUrl::UrlAdminUserStorage { realm_name, id } => format!("admin/realms/{realm_name}/user-storage/{id}/sync"),
            AdminUrl::UrlAdminServerInfo {} => "admin/serverinfo".to_string(),
            AdminUrl::UrlAdminGroups { realm_name } => format!("admin/realms/{realm_name}/groups"),
            AdminUrl::UrlAdminGroup { realm_name, id } => format!("admin/realms/{realm_name}/groups/{id}"),
            AdminUrl::UrlAdminGroupChild { realm_name, id } => format!("admin/realms/{realm_name}/groups/{id}/children"),
            AdminUrl::UrlAdminGroupPermissions { realm_name, id } => format!("admin/realms/{realm_name}/groups/{id}/management/permissions"),
            AdminUrl::UrlAdminGroupMembers { realm_name, id } => format!("admin/realms/{realm_name}/groups/{id}/members"),
            AdminUrl::UrlAdminClients { realm_name } => format!("admin/realms/{realm_name}/clients"),
            AdminUrl::UrlAdminClient { realm_name, id } => format!("admin/realms/{realm_name}/clients/{id}"),
            AdminUrl::UrlAdminClientRoles { realm_name, id } => format!("admin/realms/{realm_name}/clients/{id}/roles"),
            AdminUrl::UrlAdminClientRole { realm_name, id, role_name } => format!("admin/realms/{realm_name}/clients/{id}/roles/{role_name}"),
            AdminUrl::UrlAdminClientAuthzSettings { realm_name, id } => format!("admin/realms/{realm_name}/clients/{id}/authz/resource-server/settings"),
            AdminUrl::UrlAdminClientAuthzResources { realm_name, id } => format!("admin/realms/{realm_name}/clients/{id}/authz/resource-server/resource"),
            AdminUrl::UrlAdminClientCerts { realm_name, id, attr } => format!("admin/realms/{realm_name}/clients/{id}/certificates/{attr}"),
            AdminUrl::UrlAdminRealmRoles { realm_name } => format!("admin/realms/{realm_name}/roles"),
            AdminUrl::UrlAdminRealmImport {} => format!("admin/realms"),
            AdminUrl::UrlAdminIdps { realm_name } => format!("admin/realms/{realm_name}/identity-provider/instances"),
            AdminUrl::UrlAdminFlows { realm_name } => format!("admin/realms/{realm_name}/authentication/flows"),
            AdminUrl::UrlAdminFlowsExecutions { realm_name, flow_alias } => format!("admin/realms/{realm_name}/authentication/flows/{flow_alias}/executions"),
        };
        write!(f, "{}", str)
    }
}