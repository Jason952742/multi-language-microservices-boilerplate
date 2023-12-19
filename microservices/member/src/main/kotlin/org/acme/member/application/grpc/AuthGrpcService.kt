package org.acme.member.application.grpc

import auth.AuthService
import auth.SignInRequest
import org.acme.member.infra.service.AuthenticationService
import identity.IdentityReply
import io.quarkus.grpc.GrpcService
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.member.domain.entity.valobj.Identity
import org.acme.utils.MyScope

@GrpcService
@ExperimentalCoroutinesApi
class AuthGrpcService : AuthService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var service: AuthenticationService

    override fun login(request: SignInRequest): Uni<IdentityReply> = scope.asyncUni {
        service
            .authenticateCredentials(request)
            .awaitSuspending()
            .let {
                Identity(
                    userId = it.id ?: throw WebApplicationException("user not found", 500),
                    loginCreds = it.loginCreds
                ).toReply()
            }
    }
}
