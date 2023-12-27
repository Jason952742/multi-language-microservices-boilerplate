package org.multi_lang.domain.enums

enum class MemberStatus {
    Created,
    InUse, // If in use, cannot delete
    Subscriber,
    Expired,
    Hide,
    Disable
}
