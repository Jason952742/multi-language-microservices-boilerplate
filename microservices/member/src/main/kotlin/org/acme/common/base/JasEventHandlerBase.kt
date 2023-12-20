package org.acme.common.base

import io.grpc.Status
import io.grpc.StatusException
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure


interface JasEventHandlerBase {

    fun <D> uni(data: D): Uni<D> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun <R : JasReplayBase> uni(data: R): Uni<R> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun error(msg: String? = null, code: Int = 504): Nothing = throw StatusException(Status.UNKNOWN.withDescription("unknown error"))

}
