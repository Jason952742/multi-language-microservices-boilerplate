package org.multi_lang.domain.entity.valobj

import jakarta.persistence.Embeddable
import jakarta.persistence.EnumType
import jakarta.persistence.Enumerated
import org.multi_lang.domain.entity.enums.FunctionType
import org.multi_lang.domain.entity.enums.ModuleType


@Embeddable
data class Permission(
    @Enumerated(value = EnumType.STRING)
    var moduleType: ModuleType,

    @Enumerated(value = EnumType.STRING)
    var functionType: FunctionType,

    ) : org.shared.common.base.Desc {

    override fun toString(): String {
        return "${moduleType}-${functionType}"
    }
}
