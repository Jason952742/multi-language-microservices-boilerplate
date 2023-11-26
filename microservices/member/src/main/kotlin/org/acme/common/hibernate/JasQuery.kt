package org.acme.common.hibernate

import io.quarkus.panache.common.Sort
import org.acme.common.resource.JasIdentity

data class JasQuery(
    val filters: List<JasFilter>,
    val name: String?,
    val order_by: String?,
    val order_asc: Boolean?,
    val limit: Int,
    val offset: Int
) {

    fun sort(): Sort = Sort.by(
        this.order_by ?: "updated",
        order_asc?.let { if (it) Sort.Direction.Ascending else Sort.Direction.Descending } ?: Sort.Direction.Ascending
    )

    fun last() = this.offset + this.limit - 1

    private fun filtered() = filters.plus(
        listOf(
            JasFilter("deletion", JasFilterOp.Equal, false),
            JasFilter("name", JasFilterOp.Like, name),
            JasFilter("org_id", JasFilterOp.Equal, JasIdentity.getOrgId()),
        )
    ).filter { it.value !== null }

    fun query(): String {
        return filtered().joinToString(separator = " and ") { filterToStr(it) }
    }

    fun params(): MutableMap<String, Any> {
        val params: MutableMap<String, Any> = hashMapOf()
        filtered().forEach {
            params[it.key] = if (it.op === JasFilterOp.Like) "%${it.value!!}%" else it.value!!
        }
        return params
    }

    private fun filterToStr(filter: JasFilter): String = when (filter.op) {
        JasFilterOp.Equal -> "${filter.key} = :${filter.key}"
        JasFilterOp.NotEqual -> "${filter.key} != :${filter.key}"
        JasFilterOp.Like -> "${filter.key} like :${filter.key}"
        JasFilterOp.GreaterThan -> "${filter.key} > :${filter.key}"
        JasFilterOp.GreaterThanOrEqual -> "${filter.key} >= :${filter.key}"
        JasFilterOp.LessThanOrEqual -> "${filter.key} <= :${filter.key}"
        JasFilterOp.LessThan -> "${filter.key} < :${filter.key}"
        JasFilterOp.IsNull -> "${filter.key} is null"
        JasFilterOp.IsNotNull -> "${filter.key} is not null"
        JasFilterOp.IsNotEmpty -> "${filter.key} is not empty"
    }
}
