package org.acme.member.domain.enums

enum class MemberRoleStatus {
    Created,
    InUse, // If in use, cannot delete role
    Hide,
    Disable
}
