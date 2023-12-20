package org.acme.member.domain.entity.valobj

import auth.IdentityReply
import org.acme.common.resource.JasResult
import java.util.*

data class Identity(
    var userId: UUID,
    var loginCreds: String
) {

    private fun toProto(): auth.Identity = auth.Identity.newBuilder().also {
        it.userId = userId.toString()
        it.loginCreds = loginCreds
    }.build()

    fun toReply(): IdentityReply = IdentityReply.newBuilder().also {
        it.code = "SUCCESS"
        it.data = toProto()
    }.build()

    companion object {
        private fun fromProto(proto: auth.Identity): Identity = Identity(
            userId = UUID.fromString(proto.userId.toString()),
            loginCreds = proto.loginCreds
        )

        fun fromReply(proto: IdentityReply): JasResult<Identity> = JasResult(
            code = JasResult.ResultCode.valueOf(proto.code),
            message = if (proto.hasMessage()) proto.message else null,
            data = fromProto(proto.data)
        )
    }
}
