use std::fmt::Display;
use std::string::ToString;

#[derive(Debug)]
pub enum OpenIdUrl<'a> {
    UrlWellKnown { realm_name: &'a str },
    UrlToken { realm_name: &'a str },
    UrlUserinfo { realm_name: &'a str },
    UrlIntrospect { realm_name: &'a str },
    UrlLogout { realm_name: &'a str },
    UrlCerts { realm_name: &'a str },
    UrlEntitlement { realm_name: &'a str },
    UrlAuth { endpoint: &'a str, client_id: &'a str, redirect_uri: &'a str },
}

impl <'a> Display for OpenIdUrl<'a> {
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
pub enum AdminUrl<'a> {
    UrlAdminUsers { realm_name: &'a str },
    UrlAdminUsersCount { realm_name: &'a str },
    UrlAdminUser { realm_name: &'a str, id: &'a str },
    UrlAdminUsername { realm_name: &'a str, username: &'a str },
    UrlAdminUserConsents { realm_name: &'a str, id: &'a str },
    UrlAdminSendUpdateAccount { realm_name: &'a str, id: &'a str },
    UrlAdminSendVerifyEmail { realm_name: &'a str, id: &'a str },
    UrlAdminResetPassword { realm_name: &'a str, id: &'a str },
    UrlAdminGetSessions { realm_name: &'a str, id: &'a str },
    UrlAdminUserClientRoles { realm_name: &'a str, id: &'a str, client_id: &'a str },
    UrlAdminUserClientRolesAvailable { realm_name: &'a str, id: &'a str, client_id: &'a str },
    UrlAdminUserClientRolesComposite { realm_name: &'a str, id: &'a str, client_id: &'a str },
    UrlAdminUserRealmRoles { realm_name: &'a str, id: &'a str },
    UrlAdminUserGroup { realm_name: &'a str, id: &'a str, group_id: &'a str },
    UrlAdminUserGroups { realm_name: &'a str, id: &'a str },
    UrlAdminUserPassword { realm_name: &'a str, id: &'a str },
    UrlAdminUserStorage { realm_name: &'a str, id: &'a str },
    UrlAdminServerInfo {},
    UrlAdminGroups { realm_name: &'a str },
    UrlAdminGroup { realm_name: &'a str, id: &'a str },
    UrlAdminGroupChild { realm_name: &'a str, id: &'a str },
    UrlAdminGroupPermissions { realm_name: &'a str, id: &'a str },
    UrlAdminGroupMembers { realm_name: &'a str, id: &'a str },
    UrlAdminClients { realm_name: &'a str },
    UrlAdminClient { realm_name: &'a str, id: &'a str },
    UrlAdminClientRoles { realm_name: &'a str, id: &'a str },
    UrlAdminClientRole { realm_name: &'a str, id: &'a str, role_name: &'a str },
    UrlAdminClientAuthzSettings { realm_name: &'a str, id: &'a str },
    UrlAdminClientAuthzResources { realm_name: &'a str, id: &'a str },
    UrlAdminClientCerts { realm_name: &'a str, id: &'a str, attr: &'a str },
    UrlAdminRealmRoles { realm_name: &'a str },
    UrlAdminRealmImport {},
    UrlAdminIdps { realm_name: &'a str },
    UrlAdminFlows { realm_name: &'a str },
    UrlAdminFlowsExecutions { realm_name: &'a str, flow_alias: &'a str },
}

impl <'a> Display for AdminUrl<'a> {
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
            AdminUrl::UrlAdminRealmImport {} => "admin/realms".to_string(),
            AdminUrl::UrlAdminIdps { realm_name } => format!("admin/realms/{realm_name}/identity-provider/instances"),
            AdminUrl::UrlAdminFlows { realm_name } => format!("admin/realms/{realm_name}/authentication/flows"),
            AdminUrl::UrlAdminFlowsExecutions { realm_name, flow_alias } => format!("admin/realms/{realm_name}/authentication/flows/{flow_alias}/executions"),
        };
        write!(f, "{}", str)
    }
}