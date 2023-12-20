package org.acme.member.domain.entity.valobj


import org.acme.common.model.Gender
import org.acme.common.resource.JasResult
import auth.ProfileReply
import java.time.LocalDate
import auth.UserProfile as UserProfileProto

data class UserProfile(
    val name: String = "",
    val nickname: String = "",
    val gender: Gender? = null,
    val birth: LocalDate? = null,
    val gravatar: String = ""
) {

    private fun toProto(): UserProfileProto = UserProfileProto.newBuilder().also {
        it.name = name
        it.nickname = nickname
        gender?.run { it.gender = this.toString() }
        birth?.run { it.birth = this.toString() }
        it.gravatar = gravatar
    }.build()

    fun toReply(): ProfileReply = ProfileReply.newBuilder().also {
        it.code = "SUCCESS"
        it.data = toProto()
    }.build()

    companion object {
        private fun fromProto(proto: UserProfileProto) = UserProfile(
            nickname = proto.nickname,
            gender = if(proto.hasGender()) Gender.valueOf(proto.gender) else null,
            birth = if(proto.hasBirth()) LocalDate.parse(proto.birth) else null,
            gravatar = proto.gravatar
        )

        fun fromReply(proto: ProfileReply): JasResult<UserProfile> = JasResult(
            code = JasResult.ResultCode.valueOf(proto.code),
            message = if (proto.hasMessage()) proto.message else null,
            data = fromProto(proto.data)
        )
    }
}
