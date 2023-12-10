package org.acme.security.keycloak.authorization

import jakarta.ws.rs.GET
import jakarta.ws.rs.Path

@Path("/api/admin")
class AdminResource {
    @GET
    fun manage(): String {
        return "granted"
    }
}
