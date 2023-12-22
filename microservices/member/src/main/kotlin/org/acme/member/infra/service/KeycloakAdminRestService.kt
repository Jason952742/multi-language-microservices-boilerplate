package org.acme.member.infra.service

import jakarta.ws.rs.*
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient
import jakarta.ws.rs.core.Response
import org.acme.member.domain.keycloak.KeycloakCredentialRepresentation
import org.acme.member.domain.keycloak.KeycloakUserRepresentation
import org.jboss.resteasy.reactive.RestPath
import org.jboss.resteasy.reactive.RestQuery
import java.util.*

@Path("/admin/realms")
@RegisterRestClient(configKey = "keycloak-api")
interface KeycloakAdminRestService {

    @GET
    @Path("/multi_lang/users")
    suspend fun findUserByName(@HeaderParam("Authorization") token: String, @RestQuery username: String): Set<KeycloakUserRepresentation>

    @POST
    @Path("/multi_lang/users")
    suspend fun createUser(@HeaderParam("Authorization") token: String, user: KeycloakUserRepresentation): Response

    @PUT
    @Path("/multi_lang/users/{id}/reset-password")
    suspend fun changePassword(@HeaderParam("Authorization") token: String, @RestPath id: UUID, credential: KeycloakCredentialRepresentation): Response
}
