package org.acme.utils

import examples.HelloReply
import examples.HelloRequest
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure

object MutinyUtils {
    fun <D> uniEmit(data: D): Uni<D> = Uni.createFrom().item(data).emitOn(Infrastructure.getDefaultExecutor())

    fun <T, R> processUni(data: T, transform: (T) -> R): Uni<R> {
        return Uni.createFrom().item(data)
            .onItem().transform(transform)
    }

    fun <T> uniItem(data: T): Uni<T> = Uni.createFrom().item(data)

}
