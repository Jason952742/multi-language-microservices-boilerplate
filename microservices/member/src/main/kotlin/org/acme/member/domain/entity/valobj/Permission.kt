package org.acme.member.domain.entity.valobj

import jakarta.persistence.Embeddable
import jakarta.persistence.EnumType
import jakarta.persistence.Enumerated
import org.acme.common.base.Desc
import org.acme.member.domain.enums.FunctionType
import org.acme.member.domain.enums.ModuleType


@Embeddable
data class Permission(
    @Enumerated(value = EnumType.STRING)
    var moduleType: ModuleType,

    @Enumerated(value = EnumType.STRING)
    var functionType: FunctionType,

    ) : Desc {

    override fun toString(): String {
        return "${moduleType}-${functionType}"
    }
}
