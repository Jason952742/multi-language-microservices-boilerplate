package org.acme.common.model

import jakarta.persistence.Embeddable
import jakarta.persistence.EnumType
import jakarta.persistence.Enumerated
import jakarta.validation.constraints.NotBlank
import jakarta.validation.constraints.Past
import org.acme.common.base.Desc
import java.time.LocalDateTime

@Embeddable
data class Contacts(

    @Enumerated(value = EnumType.STRING)
    var gender: Gender? = null,

    @NotBlank(message = "not blank")
    var name: String,
    @NotBlank(message = "not blank")
    var phone: String,
    var address: String? = null,
    var company: String? = null,
    var department: String? = null,
    var position: String? = null,
    @Past(message = "must be past time")
    var birth: LocalDateTime? = null,
    var id_card: String? = null
): Desc
