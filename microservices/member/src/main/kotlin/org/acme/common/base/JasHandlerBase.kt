package org.acme.common.base

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.ws.rs.WebApplicationException
import org.acme.common.hibernate.JasEntityBase
import org.acme.common.hibernate.JasPanacheRepository
import org.acme.utils.MutinyUtils
import java.util.*

abstract class JasHandlerBase<E : JasEntityBase, C : JasCommandBase> {

    private lateinit var entity: E

    abstract suspend fun ask(id: UUID, cmd: C): Uni<E>

    abstract suspend fun add(cmd: C): Uni<E>

    private fun error(msg: String? = null, code: Int = 504): Nothing = throw WebApplicationException(msg ?: "unknown error", code)

    fun rejected(cmd: C, status: String? = null): Nothing = throw WebApplicationException("current $status status not allowed ${cmd.title} command", 403)

    suspend fun entityRef(id: UUID, repo: JasPanacheRepository<E>): E = repo.getAndLock(id).awaitSuspending().also { entity = it }

    fun insert(data: E, repo: JasPanacheRepository<E>): Uni<E> = repo.persist(data)
    fun update(data: E, repo: JasPanacheRepository<E>): Uni<E> = repo.persist(data).run { this }

    suspend fun delete(repo: JasPanacheRepository<E>): Uni<E> = repo.deleteById(entity.id).awaitSuspending().let {
        if (it) MutinyUtils.uni(entity) else throw WebApplicationException("delete ${entity.id} fail", 403)
    }

}
