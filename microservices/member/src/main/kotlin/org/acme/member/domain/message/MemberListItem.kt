package org.acme.member.domain.message

import io.quarkus.runtime.annotations.RegisterForReflection
import org.acme.common.model.MemberType
import org.acme.member.domain.enums.MemberStatus
import java.time.LocalDateTime
import java.util.*

@RegisterForReflection
data class MemberListItem(
    val id: UUID,
    val name: String,
    val nickname: String,
    val status: MemberStatus,
    val memberType: MemberType,
    val point: Long,
    val creditScore: Double,
    val level: Int,
    val lastLoginAt: LocalDateTime?,
    val updated: LocalDateTime,
    val enabled: Boolean
)
