package org.multi_lang.domain.command

sealed class MemberCommand : org.shared.common.base.JasCommandBase

data class MemberCreate(val member: org.multi_lang.domain.entity.Member, override val title: String = "Create") : MemberCommand()
data class MemberGet(override val title: String = "Get") : MemberCommand()
data class MemberDelete(override val title: String = "Delete") : MemberCommand()
data class MemberProfileChange(val nickname: String?, val description: String,  override val title: String = "Update") : MemberCommand()
