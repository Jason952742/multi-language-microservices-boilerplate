package org.acme.demo

import io.smallrye.reactive.messaging.annotations.Blocking
import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.reactive.messaging.Incoming
import org.eclipse.microprofile.reactive.messaging.Outgoing
import java.util.*

/**
 * A bean-consuming data from the "request" RabbitMQ queue and giving out a random quote.
 * The result is pushed to the "quotes" RabbitMQ exchange.
 */
@ApplicationScoped
class QuoteProcessor {
    private val random = Random()

    @Incoming("requests-c")
    @Outgoing("quotes-c")
    @Blocking
    @Throws(InterruptedException::class)
    fun process(quoteRequest: String): Quote {
        // simulate some hard-working task
        println("received id $quoteRequest form quote-request")
        Thread.sleep(200)
        val result = Quote(quoteRequest, random.nextInt(100))
        println("send $result to quotes")
        return result
    }
}
