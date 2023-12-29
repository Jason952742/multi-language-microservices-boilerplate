package org.shared.utils

object KEnum {
    inline fun <reified T : Enum<T>> printAllValues(): Array<String> {
        return enumValues<T>().map { it.name }.toTypedArray()
    }
}
