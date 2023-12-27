package org.multi_lang.domain.entity.enums

enum class MemberStatus {
    Created,
    InUse, // If in use, cannot delete
    Subscriber,
    Expired,
    Hide,
    Disable
}
