package org.acme.member.domain.keycloak

import kotlinx.serialization.Serializable

@Serializable
data class KeycloakCredentialRepresentation(
    var createdDate: Long? = null,
    var credentialData: String? = null,
    var id: String? = null,
    var priority: Int? = null,
    var secretData: String? = null,
    var temporary: Boolean? = null,
    var type: String? = null,
    var userLabel: String? = null,
    var value: String? = null,
)
