package org.acme.member.domain.entity.enums

enum class MemberRoleStatus {
    Created,
    InUse, // If in use, cannot delete role
    Hide,
    Disable
}
