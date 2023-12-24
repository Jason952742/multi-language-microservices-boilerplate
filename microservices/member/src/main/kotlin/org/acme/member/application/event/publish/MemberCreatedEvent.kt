package org.acme.member.application.event.publish

import io.quarkus.runtime.annotations.RegisterForReflection
import org.acme.common.model.MemberType
import java.util.*


@RegisterForReflection
data class MemberCreatedEvent(
    val userId: UUID,
    val userName: String,
    val memberType: MemberType,
    val memberId: UUID,
    val loginCreds: String,
    val level: Int,
    val myReferrerCode: String,
    val refereeCode: String
)
