package org.acme.member.domain.message

import member_proto.MemberResponse
import io.grpc.Status
import org.acme.common.model.MemberType
import org.acme.member.domain.enums.MemberStatus

data class MemberReply(
    val name: String,
    val nickname: String,
    val status: MemberStatus,
    val memberType: MemberType,
    val description: String
) {

    private fun toProto(): MemberResponse.Member = MemberResponse.Member.newBuilder().also {
        it.name = name
        it.nickname = nickname
        it.status = status.toString()
        it.memberType = memberType.toString()
        it.description = description
    }.build()

    fun toResponse(): MemberResponse = MemberResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.data = toProto()
    }.build()

    companion object {
        fun toError(status: Status, message: String): MemberResponse = MemberResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
        }.build()
    }
}
