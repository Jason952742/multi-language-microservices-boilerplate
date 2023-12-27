package org.multi_lang.infra.service.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class KeyCloakAccessTokenResponse(
    @SerialName("access_token")
    var accessToken: String,
    @SerialName("expires_in")
    var expiresIn: Int,
    @SerialName("refresh_expires_in")
    var refreshExpiresIn: Int,
    @SerialName("refresh_token")
    var refreshToken: String,
    @SerialName("token_type")
    var tokenType: String = "Bearer",
    @SerialName("id_token")
    var idToken: String? = null,
    @SerialName("not-before-policy")
    var notBeforePolicy: Int? = null,
    @SerialName("session_state")
    var sessionState: String? = null,
    @SerialName("scope")
    var scope: String = "openid",
    @SerialName("error")
    var error: String? = null,
    @SerialName("error_description")
    var errorDescription: String? = null,
    @SerialName("error_uri")
    var errorUri: String? = null
)
