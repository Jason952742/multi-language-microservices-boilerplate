package org.multi_lang.domain.message

import io.quarkus.runtime.annotations.RegisterForReflection
import org.shared.common.model.MemberType
import java.util.*


@RegisterForReflection
data class MemberCreatedEvent(
    val user_id: UUID,
    val user_name: String,
    val member_type: MemberType,
    val member_id: UUID,
    val login_creds: String,
    val level: Int,
    val my_referrer_code: String,
    val referee_code: String
)
