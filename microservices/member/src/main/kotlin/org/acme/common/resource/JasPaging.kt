package org.acme.common.resource

import org.acme.common.hibernate.JasQuery
import kotlin.math.floor

/**
 * Pagination Return Value
 */
data class JasPaging<T>(
    val pages: Int = 0,
    val limit: Int = 0,
    val total: Long = 0,
    val offset: Int = 0,
    val order_by: String?,
    val items: List<T>
) {
    companion object {
        fun <T> of(q: JasQuery, items: List<T>, total: Long) = JasPaging(
            offset = q.offset, limit = q.limit, order_by = q.order_by,
            pages = floor((q.offset / q.limit).toDouble()).toInt() + 1,
            items = items,
            total = total
        )
    }
}
