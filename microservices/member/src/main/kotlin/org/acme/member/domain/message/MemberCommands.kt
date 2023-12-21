package org.acme.member.domain.message

import auth.MemberUpdateRequest
import org.acme.common.base.JasCommandBase
import org.acme.member.domain.entity.Member

sealed class MemberCommand : JasCommandBase

data class MemberCreate(val member: Member, override val title: String = "Create") : MemberCommand()
data class MemberGet(override val title: String = "Get") : MemberCommand()
data class MemberDelete(override val title: String = "Delete") : MemberCommand()
data class MemberProfileChange(val nickname: String?, val description: String,  override val title: String = "Update") : MemberCommand() {
    companion object {
        fun fromProto(request: MemberUpdateRequest): MemberProfileChange = MemberProfileChange(
            nickname = request.nickname,
            description = request.description
        )
    }
}
