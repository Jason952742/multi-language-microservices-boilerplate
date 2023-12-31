package org.shared.common.resource

import org.shared.common.model.SystemDefault
import java.util.*


data class JasIdentity(
    val id: UUID,
    val name: String,
    val userId: UUID,
    val orgId: UUID? = null,
    val deptId: UUID? = null,
    val roles: List<String>,
    val key: String
) {
    companion object {
        var current: JasIdentity? = null
        fun getOrgId(): UUID = current?.orgId ?: SystemDefault.ID
    }
}
