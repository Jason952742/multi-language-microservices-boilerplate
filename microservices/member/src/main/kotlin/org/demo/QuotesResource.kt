package org.demo

import io.smallrye.mutiny.Multi
import jakarta.inject.Inject
import jakarta.ws.rs.GET
import jakarta.ws.rs.POST
import jakarta.ws.rs.Path
import jakarta.ws.rs.Produces
import jakarta.ws.rs.core.MediaType
import org.eclipse.microprofile.reactive.messaging.Channel

@Path("/quotes")
class QuotesResource {

    @Inject
    lateinit var producerService: QuoteProducer

    @Channel("quotes-p")
    lateinit var quotes: Multi<Quote>

    /**
     * Endpoint retrieving the "quotes" queue and sending the items to a server sent event.
     */
    @GET
    @Produces(MediaType.SERVER_SENT_EVENTS)
    fun stream(): Multi<Quote> {
        return quotes
    }

    /**
     * Endpoint to generate a new quote request id and send it to "quote-requests" RabbitMQ exchange using the emitter.
     */
    @POST
    @Path("/request")
    @Produces(MediaType.TEXT_PLAIN)
    fun createRequest(): String {
        val result = producerService.createRequest()
        return result
    }
}
