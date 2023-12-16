package org.acme.utils

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.asUni
import io.vertx.core.Context
import io.vertx.core.Vertx
import jakarta.enterprise.context.ApplicationScoped
import kotlinx.coroutines.*
import java.lang.Runnable
import java.util.concurrent.AbstractExecutorService
import java.util.concurrent.TimeUnit
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.EmptyCoroutineContext

class VertxCoroutineExecutor(
    private val vertxContext: Context
) : AbstractExecutorService() {

    override fun execute(command: Runnable) {
        if (Vertx.currentContext() != vertxContext) {
            vertxContext.runOnContext { command.run() }
        } else {
            command.run()
        }
    }

    override fun shutdown(): Unit = throw UnsupportedOperationException()
    override fun shutdownNow(): MutableList<Runnable> = throw UnsupportedOperationException()
    override fun isShutdown(): Boolean = throw UnsupportedOperationException()
    override fun isTerminated(): Boolean = throw UnsupportedOperationException()
    override fun awaitTermination(timeout: Long, unit: TimeUnit): Boolean = throw UnsupportedOperationException()
}

@ApplicationScoped
class MyScope : CoroutineScope {

    override val coroutineContext: CoroutineContext = SupervisorJob()

    @ExperimentalCoroutinesApi
    fun <T> asyncUni(
        context: CoroutineContext = EmptyCoroutineContext,
        start: CoroutineStart = CoroutineStart.DEFAULT,
        block: suspend CoroutineScope.() -> T
    ): Uni<T> {
        val vertxContext = checkNotNull(Vertx.currentContext())
        val dispatcher = VertxCoroutineExecutor(vertxContext).asCoroutineDispatcher()
        return async(context + dispatcher, start, block).asUni()
    }
}
