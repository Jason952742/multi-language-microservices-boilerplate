package org.demo

import io.smallrye.mutiny.Uni
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient
import org.jboss.resteasy.reactive.RestQuery
import java.util.concurrent.CompletionStage


@Path("/extensions")
@RegisterRestClient(configKey = "extensions-api")
interface ExtensionsService {
    @GET
    fun getById(@RestQuery id: String): Set<Extension>

    @GET
    fun getByIdAsync(@RestQuery id: String): CompletionStage<Set<Extension>>

    @GET
    fun getByIdAsUni(@RestQuery id: String): Uni<Set<Extension>>
}
