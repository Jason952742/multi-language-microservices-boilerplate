package org.acme.member.domain.enums

enum class MemberStatus {
    Created,
    InUse, // If in use, cannot delete
    Hide,
    Disable
}
