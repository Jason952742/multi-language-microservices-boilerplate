package org.acme.demo

import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.reactive.messaging.Channel
import org.eclipse.microprofile.reactive.messaging.Emitter
import java.util.*

/**
 * A bean-consuming data from the "request" RabbitMQ queue and giving out a random quote.
 * The result is pushed to the "quotes" RabbitMQ exchange.
 */
@ApplicationScoped
class QuoteProducer {

    @Channel("requests-p")
    private lateinit var quoteRequestEmitter: Emitter<String>

    fun createRequest() : String {
        val uuid = UUID.randomUUID()
        quoteRequestEmitter.send(uuid.toString())
        println("A producer quote request has been sent to quote-requests, id $uuid")
        return uuid.toString()
    }

//    @Incoming("quotes-p")
//    fun process(json: JsonObject): String {
//        val quote: Quote = json.mapTo(Quote::class.java)
//        // simulate some hard-working task
//
//        println("Received $quote from quotes $quote")
//        return quote.id.toString()
//    }
}
