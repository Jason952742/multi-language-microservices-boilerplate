package org.acme.common.base

import java.time.LocalDateTime
import java.util.*

data class JasApiToken(
    val id: UUID?,
    val token: String,
    val apiKey: String = "None",
    val expirationTime: LocalDateTime = LocalDateTime.now().plusDays(7)
) {
    fun isExpired() = expirationTime.isBefore(LocalDateTime.now())
}
