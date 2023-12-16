package org.acme.security.keycloak.authorization

import jakarta.ws.rs.GET
import jakarta.ws.rs.Path


@Path("/api/public")
class PublicResource {
    @GET
    fun serve() {
        // no-op
    }
}
