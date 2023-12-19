package org.acme.common.model

import jakarta.persistence.Embeddable
import jakarta.validation.constraints.NotBlank
import jakarta.validation.constraints.NotNull
import org.acme.common.base.Desc

@Embeddable
data class Location(
    @NotBlank(message = "not blank")
    var city_name: String,
    @NotBlank(message = "not blank")
    var district_name: String,
    var community_name: String? = null,
    var address: String? = null,
    @NotNull(message = "not null")
    var longitude: Double,
    @NotNull(message = "not null")
    var latitude: Double
): Desc
