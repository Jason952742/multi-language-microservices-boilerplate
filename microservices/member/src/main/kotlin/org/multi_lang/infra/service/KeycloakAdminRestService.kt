package org.multi_lang.infra.service

import io.smallrye.mutiny.Uni
import jakarta.ws.rs.*
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient
import jakarta.ws.rs.core.Response
import org.jboss.resteasy.reactive.RestPath
import org.jboss.resteasy.reactive.RestQuery
import org.multi_lang.infra.service.dto.KeycloakCredentialRepresentation
import org.multi_lang.infra.service.dto.KeycloakUserRepresentation
import java.util.*

@Path("/admin/realms")
@RegisterRestClient(configKey = "keycloak-api")
interface KeycloakAdminRestService {

    @GET
    @Path("/{realm}/users")
    suspend fun findUserByName(@HeaderParam("Authorization") token: String, @RestPath realm: String, @RestQuery username: String): Uni<Set<KeycloakUserRepresentation>>

    @POST
    @Path("/{realm}/users")
    suspend fun createUser(@HeaderParam("Authorization") token: String, @RestPath realm: String, user: KeycloakUserRepresentation): Uni<Response>

    @PUT
    @Path("/{realm}/users/{id}/reset-password")
    suspend fun changePassword(@HeaderParam("Authorization") token: String, @RestPath realm: String, @RestPath id: UUID, credential: KeycloakCredentialRepresentation): Uni<Response>
}
