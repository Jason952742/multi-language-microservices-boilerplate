package org.acme.member.application.grpc

import common_proto.ProcessResponse
import io.grpc.Status
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithSession
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import keycloak_proto.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.member.domain.entity.Member
import org.acme.member.domain.enums.IdentityMold
import org.acme.member.domain.message.ProcessReply
import org.acme.member.infra.service.AuthenticationService
import org.acme.member.infra.service.KeycloakService
import org.acme.utils.MnemonicUtil
import org.acme.utils.MyScope

@GrpcService
@ExperimentalCoroutinesApi
class KeycloakAuthGrpcService : KeycloakProtoService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var keycloakService: KeycloakService

    @Inject
    @field: Default
    lateinit var authenticationService: AuthenticationService

    @WithSession
    override fun check(request: CheckRequest): Uni<ProcessResponse> = scope.asyncUni {
        val user: Member? = authenticationService.checkMember(request.identifier).awaitSuspending()
        if (user != null) {
            ProcessReply(result = false, processedId = request.identifier).toResponse()
        } else {
            keycloakService.check(request).awaitSuspending()
        }
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val mnemonic = MnemonicUtil.generateMnemonic()

        val token = keycloakService.register(request).awaitSuspending()
        if (token.code == Status.OK.code.toString()) {
            authenticationService.register(
                mold = IdentityMold.KeyCloak,
                loginCreds = request.loginCreds,
                password = mnemonic,
                nickname = request.nickname
            ).awaitSuspending()
        }
        token
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        keycloakService.changePassword(request).awaitSuspending()
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        keycloakService.login(request)
    }

}
