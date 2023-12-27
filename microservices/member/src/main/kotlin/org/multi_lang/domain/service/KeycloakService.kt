package org.multi_lang.domain.service

import keycloak_proto.*
import common_proto.ProcessResponse
import io.grpc.Status
import io.smallrye.jwt.auth.principal.DefaultJWTCallerPrincipal
import io.smallrye.jwt.auth.principal.JWTAuthContextInfo
import io.smallrye.jwt.auth.principal.JWTCallerPrincipal
import io.smallrye.jwt.auth.principal.ParseException
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.multi_lang.application.service.KeycloakAdminRestService
import org.multi_lang.application.service.KeycloakTokenRestService
import org.shared.utils.MutinyUtils.uniItem
import org.eclipse.microprofile.config.inject.ConfigProperty
import org.eclipse.microprofile.rest.client.inject.RestClient
import org.jboss.resteasy.reactive.ClientWebApplicationException
import org.jose4j.jwt.JwtClaims
import org.jose4j.jwt.consumer.InvalidJwtException
import org.multi_lang.domain.entity.enums.GrantType
import org.multi_lang.application.grpc.assembler.KeyCloakTokenReply
import org.multi_lang.application.service.dto.KeycloakCredentialRepresentation
import org.multi_lang.application.service.dto.KeycloakUserRepresentation
import org.multi_lang.application.grpc.assembler.ProcessReply
import java.nio.charset.StandardCharsets
import java.time.LocalDateTime
import java.util.*

@ApplicationScoped
class KeycloakService {

    @RestClient
    private lateinit var keycloakService: KeycloakTokenRestService

    @RestClient
    private lateinit var keycloakAdminService: KeycloakAdminRestService

    @ConfigProperty(name = "keycloak.admin.user")
    private lateinit var keycloakAdminUser: String

    @ConfigProperty(name = "keycloak.admin.password")
    private lateinit var keycloakAdminPassword: String

    @ConfigProperty(name = "keycloak.realm")
    private lateinit var keycloakRealm: String

    @ConfigProperty(name = "keycloak.client.id")
    private lateinit var keycloakClientId: String

    @ConfigProperty(name = "keycloak.client.secret")
    private lateinit var keycloakClientSecret: String

    @Inject
    private lateinit var authContextInfo: JWTAuthContextInfo

    @Throws(ParseException::class)
    private fun parse(token: String, authContextInfo: JWTAuthContextInfo): JWTCallerPrincipal {
        try {
            // The Token has already been verified, parse the token claims only
            val json = String(Base64.getUrlDecoder().decode(token.split("\\.".toRegex()).dropLastWhile { it.isEmpty() }.toTypedArray()[1]), StandardCharsets.UTF_8)
            return DefaultJWTCallerPrincipal(JwtClaims.parse(json))
        } catch (ex: InvalidJwtException) {
            throw ParseException(ex.message)
        }
    }

    suspend fun getJwt(token: String): JWTCallerPrincipal {
        val jwt = parse(token, authContextInfo)
        // println("issuer:" + jwt.issuer)
        // println("subject:" + jwt.subject)
        // println("name:" + jwt.name)
        // println("audience:" + jwt.audience)
        // println("groups:" + jwt.groups)
        // println("expirationTime:" + jwt.expirationTime)
        // println("claimNames:" + jwt.claimNames)
        // println("issuedAtTime:" + jwt.issuedAtTime)
        // println("tokenID:" + jwt.tokenID)
        // println("realm_access:" + jwt.getClaim("realm_access"))
        // println("gender:" + jwt.getClaim("gender"))
        return jwt
    }

    private suspend fun getAdminToken(): KeyCloakTokenReply = keycloakService.getAdminToken(
        grantType = GrantType.password.toString(),
        clientId = "admin-cli",
        username = keycloakAdminUser,
        password = keycloakAdminPassword
    )

    private suspend fun getUserToken(username: String, password: String): KeyCloakTokenReply = keycloakService.getUserToken(
        realm = keycloakRealm,
        grantType = GrantType.password.toString(),
        clientId = keycloakClientId,
        clientSecret = keycloakClientSecret,
        username = username,
        password = password,
        scope = "openid"
    )

    suspend fun check(request: CheckRequest): Uni<ProcessResponse> = getAdminToken().let {
        val userResult = keycloakAdminService.findUserByName("Bearer ${it.accessToken}", keycloakRealm, request.identifier)
        uniItem(ProcessReply(result = userResult.isEmpty(), processedId = request.identifier).toResponse())
    }

    suspend fun register(request: RegistrationRequest): Uni<KeycloakTokenResponse> = getAdminToken().let {
        keycloakAdminService.findUserByName("Bearer ${it.accessToken}", keycloakRealm, request.loginCreds).run {
            if (this.isEmpty()) {
                val user = KeycloakUserRepresentation(
                    username = request.loginCreds,
                    enabled = true,
                    attributes = mapOf("expiredAt" to setOf(LocalDateTime.now().toString())),
                    credentials = listOf(
                        KeycloakCredentialRepresentation(type = "password", value = request.password, temporary = false)
                    )
                )
                val result = keycloakAdminService.createUser("Bearer ${it.accessToken}", keycloakRealm, user)
                if (result.status == 201) {
                    val userToken: KeyCloakTokenReply = getUserToken(request.loginCreds, request.password)
                    uniItem(userToken.toResponse())
                } else {
                    uniItem(KeyCloakTokenReply.toError(Status.INTERNAL, "create user failed"))
                }
            } else {
                uniItem(KeyCloakTokenReply.toError(Status.ALREADY_EXISTS, "Already registered"))
            }
        }
    }

    suspend fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = getAdminToken().let {
        val credential = KeycloakCredentialRepresentation(type = "password", value = request.newPassword, temporary = false)
        val result = keycloakAdminService.changePassword("Bearer ${it.accessToken}", keycloakRealm, UUID.fromString(request.id), credential)
        if (result.status == 204) {
            uniItem(ProcessReply(result = true, processedId = request.id).toResponse())
        } else {
            uniItem(ProcessReply.toError(Status.INTERNAL, "change password failed"))
        }
    }

    suspend fun login(request: SignInRequest): KeycloakTokenResponse = try {
        val userToken: KeyCloakTokenReply = getUserToken(request.identifier, request.password)
        userToken.toResponse()
    } catch (e: ClientWebApplicationException) {
        KeyCloakTokenReply.toError(Status.UNAUTHENTICATED, "user name or password is incorrect")
    }

}
