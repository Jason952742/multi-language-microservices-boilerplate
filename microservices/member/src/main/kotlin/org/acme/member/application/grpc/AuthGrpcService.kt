package org.acme.member.application.grpc

import auth.*
import auth.MemberResponse
import com.google.protobuf.StringValue
import io.grpc.Status
import org.acme.member.infra.service.AuthenticationService
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithSession
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.member.domain.message.MemberProfileChange
import org.acme.member.domain.handler.MemberHandler
import org.acme.member.domain.message.MemberReply
import org.acme.member.domain.message.ProcessReply
import org.acme.member.infra.search.MemberSearcher
import org.acme.utils.MyScope
import java.util.*

@GrpcService
@ExperimentalCoroutinesApi
class AuthGrpcService : AuthService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var authenticationService: AuthenticationService

    @Inject
    lateinit var searcher: MemberSearcher

    @Inject
    lateinit var memberHandler: MemberHandler

    @WithSession
    override fun login(request: SignInRequest): Uni<IdentityResponse> = scope.asyncUni {
        authenticationService.authenticateCredentials(request).awaitSuspending()
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<IdentityResponse> = scope.asyncUni {
        authenticationService.register(request).awaitSuspending()
    }

    @WithSession
    override fun getMember(request: StringValue): Uni<MemberResponse> = scope.asyncUni {
        searcher.getById(UUID.fromString(request.value)).awaitSuspending().let {
            MemberReply(
                name = it.name,
                nickname = it.nickname,
                gender = it.gender,
                birth = it.birth,
                gravatar = it.gravatar
            ).toResponse()
        }
    }

    @WithTransaction
    override fun updateMember(request: MemberUpdateRequest): Uni<MemberResponse> = scope.asyncUni {
        val cmd = MemberProfileChange.fromProto(request)
        memberHandler.ask(id = UUID.fromString(request.id), cmd = cmd).awaitSuspending().let {
            MemberReply(
                name = it.name,
                nickname = it.nickname,
                gender = it.gender,
                birth = it.birth,
                gravatar = it.gravatar
            ).toResponse()
        }
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        if (request.newPassword == request.confirm) {
            authenticationService.changePassword(UUID.fromString(request.id), request).awaitSuspending()
        } else ProcessReply.toError(Status.UNAUTHENTICATED, "New password and confirmation password do not match")
    }
}
