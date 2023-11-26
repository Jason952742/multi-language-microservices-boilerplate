package org.acme

import org.acme.utils.EncryptionUtils
import io.quarkus.runtime.Quarkus
import io.quarkus.runtime.QuarkusApplication
import io.quarkus.runtime.annotations.QuarkusMain
import org.eclipse.microprofile.config.inject.ConfigProperty

@QuarkusMain
object Main {

    @JvmStatic
    fun main(args: Array<String>) {
        println("App Start...")
        Quarkus.run(MyApp::class.java, *args)
    }

    class MyApp : QuarkusApplication {

        @ConfigProperty(name = "secretKey")
        lateinit var key: String

        @Throws(Exception::class)
        override fun run(vararg args: String): Int {
            EncryptionUtils.setKey(key)
            println("App running...")
            Quarkus.waitForExit()
            return 0
        }
    }
}
