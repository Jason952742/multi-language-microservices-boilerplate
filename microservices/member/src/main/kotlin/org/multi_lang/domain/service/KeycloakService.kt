package org.multi_lang.domain.service

import io.grpc.Status
import io.grpc.StatusException
import io.smallrye.jwt.auth.principal.DefaultJWTCallerPrincipal
import io.smallrye.jwt.auth.principal.JWTAuthContextInfo
import io.smallrye.jwt.auth.principal.JWTCallerPrincipal
import io.smallrye.jwt.auth.principal.ParseException
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.multi_lang.infra.service.KeycloakAdminRestService
import org.multi_lang.infra.service.KeycloakTokenRestService
import org.eclipse.microprofile.config.inject.ConfigProperty
import org.eclipse.microprofile.rest.client.inject.RestClient
import org.jose4j.jwt.JwtClaims
import org.jose4j.jwt.consumer.InvalidJwtException
import org.multi_lang.domain.entity.enums.GrantType
import org.multi_lang.infra.service.dto.KeycloakAccessToken
import org.multi_lang.infra.service.dto.KeycloakCredential
import org.multi_lang.infra.service.dto.KeycloakUser
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

    private suspend fun getAdminToken(): KeycloakAccessToken = keycloakService.getAdminToken(
        grantType = GrantType.password.toString(),
        clientId = "admin-cli",
        username = keycloakAdminUser,
        password = keycloakAdminPassword
    )

    suspend fun getUserToken(identifier: String, password: String): KeycloakAccessToken = keycloakService.getUserToken(
        realm = keycloakRealm,
        grantType = GrantType.password.toString(),
        clientId = keycloakClientId,
        clientSecret = keycloakClientSecret,
        username = identifier,
        password = password,
        scope = "openid"
    )

    suspend fun check(identifier: String): Uni<Set<KeycloakUser>> = getAdminToken().let {
        val userResult = keycloakAdminService.findUserByName("Bearer ${it.accessToken}", keycloakRealm, identifier)
        Uni.createFrom().item(userResult)
    }

    suspend fun register(loginCreds: String, password: String): Uni<KeycloakAccessToken> = getAdminToken().let {
        keycloakAdminService.findUserByName("Bearer ${it.accessToken}", keycloakRealm, loginCreds).run {
            if (this.isEmpty()) {
                val user = KeycloakUser(
                    username = loginCreds,
                    enabled = true,
                    attributes = mapOf("expiredAt" to setOf(LocalDateTime.now().toString())),
                    credentials = listOf(
                        KeycloakCredential(type = "password", value = password, temporary = false)
                    )
                )
                val result = keycloakAdminService.createUser("Bearer ${it.accessToken}", keycloakRealm, user)
                if (result.status == 201) {
                    val token = getUserToken(loginCreds, password)
                    Uni.createFrom().item(token)
                } else {
                    Uni.createFrom().failure(StatusException(Status.INTERNAL.withDescription("create user failed")))
                }
            } else {
                Uni.createFrom().failure(StatusException(Status.ALREADY_EXISTS.withDescription("Already registered")))
            }
        }
    }

    suspend fun changePassword(id: UUID, newPassword: String): Uni<String> = getAdminToken().let {
        val credential = KeycloakCredential(type = "password", value = newPassword, temporary = false)
        val result = keycloakAdminService.changePassword("Bearer ${it.accessToken}", keycloakRealm, id, credential)
        if (result.status == 204) {
            Uni.createFrom().item(id.toString())
        } else {
            Uni.createFrom().failure(StatusException(Status.INTERNAL.withDescription("change password failed")))
        }
    }

    suspend fun login(identifier: String, password: String): Uni<KeycloakAccessToken> = getUserToken(identifier, password).let {
        Uni.createFrom().item(it)
    }

}
