package org.multi_lang.domain.entity.valobj

import jakarta.persistence.Embeddable
import jakarta.persistence.EnumType
import jakarta.persistence.Enumerated
import org.multi_lang.domain.enums.MemberRoleType
import java.time.LocalDateTime


@Embeddable
data class MemberRole(
    @Enumerated(value = EnumType.STRING)
    var role: MemberRoleType,

    var expired: LocalDateTime
): org.shared.common.base.Desc {

    fun active(): Boolean {
        return LocalDateTime.now().isBefore(this.expired)
    }
}
