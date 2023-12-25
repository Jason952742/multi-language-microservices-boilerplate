package org.acme.utils

import java.util.*

object StrUtils {
    fun toSnakeCase(name: String): String {
        return name
            .replace(Regex("([a-z])([A-Z]+)"), "$1_$2")
            .lowercase(Locale.getDefault())
    }
}
