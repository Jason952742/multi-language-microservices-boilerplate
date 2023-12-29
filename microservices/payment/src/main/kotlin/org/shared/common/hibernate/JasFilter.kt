package org.shared.common.hibernate

data class JasFilter(
    val key: String = "?",
    val op: JasFilterOp = JasFilterOp.Equal,
    val value: Any?
) {
    companion object {
        fun strEq(k: String, v: Any) = JasFilter(key = k, value = listOf(v))
    }
}
