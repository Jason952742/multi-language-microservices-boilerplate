package org.acme.utils

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure

object MutinyUtils {
    fun <D> uniEmit(data: D): Uni<D> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun <T> uni(data: T): Uni<T> = Uni.createFrom().item(data)
}
