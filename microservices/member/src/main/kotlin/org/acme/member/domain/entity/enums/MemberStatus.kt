package org.acme.member.domain.entity.enums

enum class MemberStatus {
    Created,
    InUse, // If in use, cannot delete
    Hide,
    Disable
}
