package org.multi_lang.domain.entity.item

import io.quarkus.runtime.annotations.RegisterForReflection
import org.multi_lang.domain.entity.enums.MemberStatus
import org.shared.common.model.MemberType
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
