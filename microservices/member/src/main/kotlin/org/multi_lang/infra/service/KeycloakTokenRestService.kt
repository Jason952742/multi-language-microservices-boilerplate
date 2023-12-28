package org.multi_lang.infra.service

import io.smallrye.mutiny.Uni
import jakarta.ws.rs.*
import org.eclipse.microprofile.rest.client.inject.RegisterRestClient
import jakarta.ws.rs.core.MediaType
import org.jboss.resteasy.reactive.RestPath
import org.multi_lang.infra.service.dto.KeycloakAccessToken

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
    ): KeycloakAccessToken

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
    ): KeycloakAccessToken

}
