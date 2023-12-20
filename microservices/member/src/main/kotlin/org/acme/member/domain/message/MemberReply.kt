package org.acme.member.domain.message

import org.acme.common.model.Gender
import auth.MemberResponse
import java.time.LocalDate
import io.grpc.Status

data class MemberReply(
    val name: String = "",
    val nickname: String = "",
    val gender: Gender? = null,
    val birth: LocalDate? = null,
    val gravatar: String = ""
) {

    private fun toProto(): MemberResponse.Member = MemberResponse.Member.newBuilder().also {
        it.name = name
        it.nickname = nickname
        gender?.run { it.gender = this.toString() }
        birth?.run { it.birth = this.toString() }
        it.gravatar = gravatar
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
