package org.acme.member.domain.message

import io.quarkus.runtime.annotations.RegisterForReflection
import java.time.LocalDateTime
import java.util.*
import org.acme.member.domain.enums.IdentityMold

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
