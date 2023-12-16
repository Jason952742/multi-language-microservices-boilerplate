package org.acme.security.keycloak.authorization

import io.quarkus.security.identity.SecurityIdentity
import jakarta.inject.Inject
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path


@Path("/api/users")
class UserResource {
    @Inject
    lateinit var keycloakSecurityContext: SecurityIdentity

    @GET
    @Path("/me")
    fun me(): User {
        return User(keycloakSecurityContext)
    }

    class User internal constructor(securityContext: SecurityIdentity?) {
        val userName: String = securityContext!!.principal.name
    }
}
