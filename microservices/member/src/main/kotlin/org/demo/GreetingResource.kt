package org.demo

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure
import jakarta.inject.Inject
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path
import jakarta.ws.rs.Produces
import jakarta.ws.rs.core.MediaType

@Path("/hello")
class GreetingResource {

    @Inject
    lateinit var service: GreetingService

    @GET
    @Produces(MediaType.TEXT_PLAIN)
    @Path("/greeting/{name}")
    fun greeting(name: String): Uni<String> {
        return service.greeting(name)
    }

    @GET
    @Produces(MediaType.TEXT_PLAIN)
    fun hello(): Uni<String> {
        return Uni.createFrom().item { "Hello from RESTEasy Reactive" }
            .emitOn(Infrastructure.getDefaultExecutor())
    }

}
