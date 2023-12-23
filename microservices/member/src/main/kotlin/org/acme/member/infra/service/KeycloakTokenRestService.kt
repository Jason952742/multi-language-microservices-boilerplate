package org.acme.member.infra.service

import jakarta.ws.rs.*
import org.acme.member.domain.keycloak.KeyCloakTokenReply
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient
import jakarta.ws.rs.core.MediaType
import org.jboss.resteasy.reactive.RestPath

@Path("/realms")
@RegisterRestClient(configKey = "keycloak-api")
interface KeycloakTokenRestService {

    @POST
    @Consumes(MediaType.APPLICATION_FORM_URLENCODED)
    @Path("/master/protocol/openid-connect/token")
    suspend fun getAdminToken(
        @FormParam("grant_type") grantType: String,
        @FormParam("client_id") clientId: String,
        @FormParam("username") username: String,
        @FormParam("password") password: String,
    ): KeyCloakTokenReply

    @POST
    @Consumes(MediaType.APPLICATION_FORM_URLENCODED)
    @Path("/{realm}/protocol/openid-connect/token")
    suspend fun getUserToken(
        @RestPath realm: String,
        @FormParam("grant_type") grantType: String,
        @FormParam("client_id") clientId: String,
        @FormParam("client_secret") clientSecret: String,
        @FormParam("username") username: String,
        @FormParam("password") password: String,
        @FormParam("scope") scope: String,
    ): KeyCloakTokenReply

}
