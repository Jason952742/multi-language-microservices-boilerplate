package org.multi_lang.application.service.dto

import kotlinx.serialization.Serializable
import org.multi_lang.application.service.dto.KeycloakCredentialRepresentation

@Serializable
data class KeycloakUserRepresentation(
    var id: String? = null,
    var createdTimestamp: Long? = null,
    var username: String? = null,
    var enabled: Boolean? = null,
    var totp: Boolean? = null,
    var emailVerified: Boolean? = null,
    var disableableCredentialTypes: Set<String>? = null,
    var requiredActions: Set<(String)>? = null,
    var notBefore: Long? = null,
    var access: Map<String, Boolean>? = null,
    // optional from response
    var attributes: Map<String, Set<String>>? = null,
    var credentials: List<KeycloakCredentialRepresentation>? = null,
    var email: String? = null,
    var federationLink: String? = null,
    var firstName: String? = null,
    var groups: Set<String>? = null,
    var lastName: String? = null,
    var origin: String? = null,
    var realmRoles: Set<String>? = null,
    var self: String? = null,
    var serviceAccountClientId: String? = null,
) {

}
