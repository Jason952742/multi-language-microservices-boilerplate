package org.acme

import io.quarkus.runtime.ShutdownEvent
import io.quarkus.runtime.StartupEvent
import jakarta.enterprise.context.ApplicationScoped
import jakarta.enterprise.event.Observes
import jakarta.inject.Inject
import org.slf4j.Logger
import org.slf4j.LoggerFactory


@ApplicationScoped
class AppLifecycleBean {

    val RESET = "\u001B[0m"
    val CYAN = "\u001B[36m"
    val ORANGE = "\u001B[38;5;208m"

    @Inject
    private lateinit var consulService: ConsulService

    /**
     * Inject a bean used in the callbacks.
     */
    @Inject
    private lateinit var bean: MyOtherBean

    fun onStart(@Observes ev: StartupEvent?) {
        LOGGER.info("$CYAN The application is starting...{} $RESET", bean.hello())

        consulService.registerServiceWithHealthCheck("member-microservice")
    }

    fun onStop(@Observes ev: ShutdownEvent?) {
        LOGGER.info("$ORANGE The application is stopping... {} $RESET", bean.bye())
    }

    companion object {
        private val LOGGER: Logger = LoggerFactory.getLogger("ListenerBean")
    }
}
