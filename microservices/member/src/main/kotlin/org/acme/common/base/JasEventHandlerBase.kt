package org.acme.common.base

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure
import jakarta.ws.rs.WebApplicationException


interface JasEventHandlerBase {

    fun <D> uni(data: D): Uni<D> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun <R : JasReplayBase> uni(data: R): Uni<R> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun error(msg: String? = null, code: Int = 504): Nothing = throw WebApplicationException( msg ?: "unknown error", code)

}
