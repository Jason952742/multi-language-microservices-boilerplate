package org.shared.common.base

import io.grpc.Status
import io.grpc.StatusException
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import org.shared.common.hibernate.JasEntityBase
import org.shared.common.hibernate.JasPanacheRepository
import org.shared.utils.MutinyUtils
import java.util.*

abstract class JasHandlerBase<E : JasEntityBase, C : org.shared.common.base.JasCommandBase> {

    lateinit var entity: E

    abstract suspend fun ask(id: UUID, cmd: C): Uni<E>

    abstract suspend fun add(cmd: C): Uni<E>

    private fun error(msg: String? = null): Nothing = throw StatusException(Status.INVALID_ARGUMENT.withDescription(msg ?: "Invalid argument."))
    fun rejected(cmd: C, status: String? = null): Nothing = throw StatusException(Status.INVALID_ARGUMENT.withDescription("current $status status not allowed ${cmd.title} command"))

    suspend fun entityRef(id: UUID, repo: JasPanacheRepository<E>): E {
        val m = repo.get(id).awaitSuspending()
        // val m = repo.getAndLock(id).awaitSuspending()
        this.entity = m
        return m
    }

    fun insert(data: E, repo: JasPanacheRepository<E>): Uni<E> = repo.persist(data)
    fun update(data: E, repo: JasPanacheRepository<E>): Uni<E> {
        val m = repo.persist(data)
        return m
    }

    suspend fun delete(repo: JasPanacheRepository<E>): Uni<E> = repo.deleteById(entity.id).awaitSuspending().let {
        if (it) MutinyUtils.uniItem(entity) else throw StatusException(Status.DATA_LOSS.withDescription("delete ${entity.id} fail"))
    }

}
