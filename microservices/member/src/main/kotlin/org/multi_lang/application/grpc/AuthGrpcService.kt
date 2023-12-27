package org.multi_lang.application.grpc

import auth_proto.*
import common_proto.ProcessResponse
import io.grpc.Status
import org.multi_lang.domain.service.AuthenticationService
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithSession
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.multi_lang.domain.entity.enums.IdentityMold
import org.multi_lang.application.grpc.assembler.ProcessReply
import org.shared.utils.MyScope
import java.util.*

@GrpcService
@ExperimentalCoroutinesApi
class AuthGrpcService : AuthProtoService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var authenticationService: AuthenticationService

    @WithSession
    override fun check(request: CheckRequest): Uni<ProcessResponse> = scope.asyncUni {
        val loginPasses = authenticationService.checkLoginPasses(IdentityMold.valueOf(request.mold), request.identifier).awaitSuspending()
        ProcessReply(result = loginPasses == null, processedId = request.identifier).toResponse()
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<IdentityResponse> = scope.asyncUni {
        authenticationService.register(
            mold = IdentityMold.valueOf(request.mold),
            loginCreds = request.loginCreds,
            password = request.password,
            userId = UUID.randomUUID(), // TODO: get userID
            nickname = request.nickname
        ).awaitSuspending()
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<IdentityResponse> = scope.asyncUni {
        authenticationService.authenticateCredentials(request).awaitSuspending()
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        if (request.newPassword == request.confirm) {
            authenticationService.changePassword(UUID.fromString(request.id), request).awaitSuspending()
        } else ProcessReply.toError(Status.UNAUTHENTICATED, "New password and confirmation password do not match")
    }

}
