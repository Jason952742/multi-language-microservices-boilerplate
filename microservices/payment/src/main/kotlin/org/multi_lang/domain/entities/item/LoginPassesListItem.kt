package org.multi_lang.domain.entity.item

import io.quarkus.runtime.annotations.RegisterForReflection
import java.time.LocalDateTime
import java.util.*
import org.multi_lang.domain.entity.enums.IdentityMold

@RegisterForReflection
data class LoginPassesListItem(
    val id: UUID,
    val name: String,
    val loginCreds: String,
    val mold: IdentityMold,
    val identifier: String,
    val expired: LocalDateTime,
    val updated: LocalDateTime,
    val enabled: Boolean
)
