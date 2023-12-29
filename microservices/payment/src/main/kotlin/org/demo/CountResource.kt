package org.demo

import jakarta.inject.Inject
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path
import jakarta.ws.rs.Produces
import jakarta.ws.rs.core.MediaType


@Path("/count")
class CountResource {
    @Inject
    lateinit var counter: CounterBean

    @GET
    @Produces(MediaType.TEXT_PLAIN)
    fun hello(): String {
        return "count: " + counter.get()
    }
}
