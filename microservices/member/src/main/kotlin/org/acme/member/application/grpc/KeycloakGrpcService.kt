package org.acme.member.application.grpc

import keycloak_proto.*
import common_proto.ProcessResponse
import io.grpc.Status
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithSession
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import jakarta.inject.Inject
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.member.domain.keycloak.KeycloakCredentialRepresentation
import org.acme.member.domain.keycloak.GrantType
import org.acme.member.domain.keycloak.KeyCloakTokenReply
import org.acme.member.domain.keycloak.KeycloakUserRepresentation
import org.acme.member.domain.message.ProcessReply
import org.acme.member.infra.service.KeycloakAdminRestService
import org.acme.member.infra.service.KeycloakTokenRestService
import org.acme.utils.MyScope
import org.eclipse.microprofile.rest.client.inject.RestClient
import org.jboss.resteasy.reactive.ClientWebApplicationException
import java.util.*

@GrpcService
@ExperimentalCoroutinesApi
class KeycloakGrpcService : KeycloakProtoService {

    @Inject
    lateinit var scope: MyScope

    @RestClient
    lateinit var keycloakService: KeycloakTokenRestService

    @RestClient
    lateinit var keycloakAdminService: KeycloakAdminRestService

    private suspend fun getAdminToken(): KeyCloakTokenReply = keycloakService.getAdminToken(
        grantType = GrantType.password.toString(),
        clientId = "admin-cli",
        username = "admin",
        password = "adminpassword"
    )

    private suspend fun getUserToken(username: String, password: String): KeyCloakTokenReply = keycloakService.getUserToken(
        grantType = GrantType.password.toString(),
        clientId = "web-auth-client",
        clientSecret = "eHuKZX0iGqKB7glv0T5yGFqwjK38zjS9",
        username = username,
        password = password,
        scope = "openid"
    )

    @WithSession
    override fun check(request: CheckRequest): Uni<ProcessResponse> = scope.asyncUni {
        val token = getAdminToken()
        val userResult = keycloakAdminService.findUserByName("Bearer ${token.accessToken}", request.identifier)
        ProcessReply(result = userResult.isEmpty(), processedId = request.identifier).toResponse()
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val token = getAdminToken()
        keycloakAdminService.findUserByName("Bearer ${token.accessToken}", request.loginCreds).run {
            if (this.isEmpty()) {
                val user = KeycloakUserRepresentation(
                    username = request.loginCreds,
                    enabled = true,
                    credentials = listOf(KeycloakCredentialRepresentation(type = "password", value = request.password, temporary = false))
                )
                val result = keycloakAdminService.createUser("Bearer ${token.accessToken}", user)
                if (result.status == 201) {
                    val userToken: KeyCloakTokenReply = getUserToken(request.loginCreds, request.password)
                    userToken.toResponse()
                } else {
                    KeyCloakTokenReply.toError(Status.INTERNAL, "create user failed")
                }
            } else {
                KeyCloakTokenReply.toError(Status.ALREADY_EXISTS, "Already registered")
            }
        }
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        if (request.newPassword == request.confirm) {
            val token = getAdminToken()
            val credential = KeycloakCredentialRepresentation(type = "password", value = request.newPassword, temporary = false)
            val result = keycloakAdminService.changePassword("Bearer ${token.accessToken}", UUID.fromString(request.id), credential)
            if (result.status == 204) {
                ProcessReply(result = true, processedId = request.id).toResponse()
            } else {
                ProcessReply.toError(Status.INTERNAL, "change password failed")
            }
        } else ProcessReply.toError(Status.UNAUTHENTICATED, "New password and confirmation password do not match")
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        try {
            val userToken: KeyCloakTokenReply = getUserToken(request.identifier, request.password)
            userToken.toResponse()
        } catch (e: ClientWebApplicationException) {
            KeyCloakTokenReply.toError(Status.UNAUTHENTICATED, "user name or password is incorrect")
        }
    }

}
