package org.acme.member.application.grpc

import identity.IdentityReply
import io.quarkus.grpc.GrpcService
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.member.infra.service.RegisterService
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import org.acme.member.domain.entity.valobj.Identity
import org.acme.utils.MyScope
import registration.RegistrationRequest
import registration.RegistrationService

@GrpcService
@ExperimentalCoroutinesApi
class RegistrationGrpcService : RegistrationService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var service: RegisterService

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<IdentityReply> = scope.asyncUni {
        service
            .register(request)
            .awaitSuspending()
            .let {
                Identity(
                    userId = it.id ?: throw WebApplicationException("user not found", 500),
                    loginCreds = it.loginCreds
                ).toReply()
            }
    }
}
