package org.acme.common.resource

import java.util.*


data class JasIdentity(
    val id: UUID,
    val name: String,
    val user_id: UUID,
    val org_id: UUID? = null,
    val dept_id: UUID? = null,
    val roles: List<String>,
    val key: String
) {
    companion object {
        var current: JasIdentity? = null
        fun getOrgId(): UUID = current?.org_id ?: UUID.fromString("88888888-8888-8888-8888-888888888888")
    }
}
