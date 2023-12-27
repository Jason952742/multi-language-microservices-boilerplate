package org.multi_lang.domain.message

import member_proto.MemberUpdateRequest
import org.shared.common.base.JasCommandBase
import org.multi_lang.domain.entity.Member

sealed class MemberCommand : org.shared.common.base.JasCommandBase

data class MemberCreate(val member: org.multi_lang.domain.entity.Member, override val title: String = "Create") : MemberCommand()
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
