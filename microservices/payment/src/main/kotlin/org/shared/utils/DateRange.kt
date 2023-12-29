package org.shared.utils

import java.time.LocalDate

data class DateRange(val first: LocalDate, val last: LocalDate, val isFullMonth: Boolean, val rate: Double)
