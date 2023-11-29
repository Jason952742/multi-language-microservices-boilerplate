package org.acme.utils

object KEnum {
    inline fun <reified T : Enum<T>> printAllValues(): Array<String> {
        return enumValues<T>().map { it.name }.toTypedArray()
    }
}
