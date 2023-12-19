package org.acme.common.model

import java.time.LocalDateTime
import java.util.*
import jakarta.persistence.Embeddable

@Embeddable
data class MIDetail(
    var party_id: UUID? = null,
    var party_name: String? = null,
    var party_type: String? = null,
    var thing_id: UUID? = null,
    var thing_name: String? = null,
    var thing_type: String? = null,
    var place_id: UUID? = null,
    var place_name: String? = null,
    var place_type: String? = null,
    var mi_id: UUID? = null,
    var mi_name: String? = null,
    var mi_type: String? = null,
    var reason: String? = null,
    var desc: String? = null,
    var belong_from: LocalDateTime? = null,
    var belong_to: LocalDateTime? = null
)
