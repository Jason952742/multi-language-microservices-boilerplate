package org.acme.member.domain.message

import auth.MemberUpdateRequest
import org.acme.common.base.JasCommandBase
import org.acme.common.model.Gender
import org.acme.member.domain.entity.Member
import java.time.LocalDate

sealed class MemberCommand : JasCommandBase

data class MemberCreate(val member: Member, override val title: String = "Create") : MemberCommand()
data class MemberGet(override val title: String = "Get") : MemberCommand()
data class MemberDelete(override val title: String = "Delete") : MemberCommand()
data class MemberProfileChange(val nickname: String?, val gender: Gender?, val birth: LocalDate?, val gravatar: String?, override val title: String = "Update") : MemberCommand() {
    companion object {
        fun fromProto(request: MemberUpdateRequest): MemberProfileChange = MemberProfileChange(
            nickname = request.nickname,
            gender = if (request.hasGender()) Gender.valueOf(request.gender) else null,
            birth = if (request.hasBirth()) LocalDate.parse(request.birth) else null,
            gravatar = if (request.hasGravatar()) request.gravatar else null
        )
    }
}
