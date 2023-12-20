package org.acme.member.domain.message

import auth.IdentityResponse
import java.util.*
import io.grpc.Status

data class IdentityReply(
    var userId: UUID,
    var loginCreds: String
) {

    private fun toProto(): IdentityResponse.Identity = IdentityResponse.Identity.newBuilder().also {
        it.userId = userId.toString()
        it.loginCreds = loginCreds
    }.build()

    fun toResponse(): IdentityResponse = IdentityResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.data = toProto()
    }.build()

    companion object {
        fun toError(status: Status, message: String): IdentityResponse = IdentityResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
        }.build()
    }
}
