package org.multi_lang.infra.service.dto

import kotlinx.serialization.Serializable

@Serializable
data class KeycloakCredential(
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
