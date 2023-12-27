package org.multi_lang.application.grpc.assembler

import io.grpc.Status
import keycloak_proto.KeycloakTokenResponse
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class KeyCloakTokenReply(
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
    var scope: String = "openid"
) {

    companion object {
        fun toError(status: Status, message: String): KeycloakTokenResponse = KeycloakTokenResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
        }.build()
    }
}
