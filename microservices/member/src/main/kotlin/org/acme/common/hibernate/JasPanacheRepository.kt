package org.acme.common.hibernate

import io.grpc.Status
import io.grpc.StatusException
import io.quarkus.hibernate.reactive.panache.PanacheQuery
import io.quarkus.hibernate.reactive.panache.PanacheRepositoryBase
import io.quarkus.panache.common.Parameters
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.persistence.LockModeType
import org.acme.common.resource.JasPaging
import org.acme.utils.MutinyUtils.uniItem
import java.util.*

interface JasPanacheRepository<T : JasEntityBase> : PanacheRepositoryBase<T, UUID> {

    suspend fun get(id: UUID): Uni<T> = findById(id) ?: throw StatusException(Status.NOT_FOUND.withDescription("$id non-existent..."))
    suspend fun getAndLock(id: UUID): Uni<T> = findById(id, LockModeType.PESSIMISTIC_WRITE) ?: throw StatusException(Status.NOT_FOUND.withDescription("$id non-existent..."))
    suspend fun getOrNull(id: UUID?): Uni<T>? = id?.let { findById(id) }
    suspend fun getIn(ids: List<UUID>): Uni<MutableList<T>> = list("id IN (:ids)", Parameters.with("ids", ids))
    suspend fun getInOrNull(ids: List<UUID>?): Uni<MutableList<T>>? = ids?.let { getIn(ids) }

    suspend fun findByName(name: String): Uni<T>? = find("name", name).firstResult()
    suspend fun deleteByName(name: String): Uni<Long>? = delete("name", name)

    suspend fun count(q: JasQuery): Uni<Long> {
        val p = find(q.query(), q.sort(), q.params())
        return p.count()
    }

    suspend fun list(q: JasQuery): Uni<List<T>> {
        val p: PanacheQuery<T> = find(q.query(), q.sort(), q.params())
        return p.range<T>(q.offset, q.last()).list()
    }

    suspend fun listAndCount(q: JasQuery): Uni<JasPaging<T>> {
        val total: Long = count(q).awaitSuspending()
        val items: List<T> = list(q).awaitSuspending()
        val result = JasPaging.of(q = q, items = items, total = total)
        return uniItem(result)
    }
}
