package org.acme.member.domain.entity.valobj

import jakarta.persistence.Embeddable
import jakarta.persistence.EnumType
import jakarta.persistence.Enumerated
import org.acme.common.base.Desc
import org.acme.member.domain.enums.MemberRoleType
import java.time.LocalDateTime


@Embeddable
data class MemberRole(
    @Enumerated(value = EnumType.STRING)
    var role: MemberRoleType,

    var expired: LocalDateTime
): Desc {

    fun active(): Boolean {
        return LocalDateTime.now().isBefore(this.expired)
    }
}
