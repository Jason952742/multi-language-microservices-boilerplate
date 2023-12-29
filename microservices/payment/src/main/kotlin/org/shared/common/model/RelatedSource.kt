package org.shared.common.model

import java.time.LocalDateTime
import java.util.*
import jakarta.persistence.Embeddable

@Embeddable
data class RelatedSource(
    var mold: String,
    var party_name: String,
    var party_id: UUID,
    var thing_name: String? = null,
    var thing_id: UUID? = null,
    var related_time: LocalDateTime
)
