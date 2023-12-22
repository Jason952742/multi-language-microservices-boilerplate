package org.acme.member.domain.keycloak

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
    private fun toProto(): KeycloakTokenResponse.KeycloakToken = KeycloakTokenResponse.KeycloakToken.newBuilder().also {
        it.accessToken = accessToken
        it.expiresIn = expiresIn
        it.refreshExpiresIn = refreshExpiresIn
        it.refreshToken = refreshToken
        it.tokenType = tokenType
    }.build()

    fun toResponse(): KeycloakTokenResponse = KeycloakTokenResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.data = toProto()
    }.build()

    companion object {
        fun toError(status: Status, message: String): KeycloakTokenResponse = KeycloakTokenResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
        }.build()
    }
}
