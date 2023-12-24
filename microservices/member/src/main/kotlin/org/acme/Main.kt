package org.acme

import io.quarkus.runtime.Quarkus
import io.quarkus.runtime.QuarkusApplication
import io.quarkus.runtime.annotations.QuarkusMain
import org.acme.utils.EncryptionUtils
import org.eclipse.microprofile.config.inject.ConfigProperty
import org.jboss.logging.Logger
import java.util.*

@QuarkusMain
object Main {

    val logger: Logger = Logger.getLogger("MainLogger")
    const val RESET = "\u001B[0m"
    const val PINK = "\u001B[38;5;206m"

    @JvmStatic
    fun main(args: Array<String>) {
        logger.info("$PINK App Starting... $RESET")
        Quarkus.run(MyApp::class.java, *args)
    }

    class MyApp : QuarkusApplication {

        @ConfigProperty(name = "secretKey")
        lateinit var key: String

        @Throws(Exception::class)
        override fun run(vararg args: String): Int {
            EncryptionUtils.setKey(key)
            logger.info("$PINK App Running... $RESET")
            Quarkus.waitForExit()
            return 0
        }

    }
}
