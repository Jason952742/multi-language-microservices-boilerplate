package org.acme.member.application.grpc

import auth.IdentityReply
import auth.AuthService
import auth.RegistrationRequest
import auth.SignInRequest
import auth.ProfileReply
import auth.ProfileRequest
import auth.PasswordChangeRequest
import com.google.protobuf.BoolValue
import com.google.protobuf.StringValue
import org.acme.member.infra.service.AuthenticationService
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.common.model.Gender
import org.acme.member.domain.commands.MemberProfileChange
import org.acme.member.domain.entity.valobj.Identity
import org.acme.member.domain.entity.valobj.UserProfile
import org.acme.member.domain.handler.MemberHandler
import org.acme.member.infra.search.MemberSearcher
import org.acme.utils.MyScope
import java.time.LocalDate
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

    override fun login(request: SignInRequest): Uni<IdentityReply> = scope.asyncUni {
        authenticationService
            .authenticateCredentials(request)
            .awaitSuspending()
            .let {
                Identity(
                    userId = it.id ?: throw WebApplicationException("user not found", 500),
                    loginCreds = it.loginCreds
                ).toReply()
            }
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<IdentityReply> = scope.asyncUni {
        authenticationService
            .register(request)
            .awaitSuspending()
            .let {
                Identity(
                    userId = it.id ?: throw WebApplicationException("user not found", 500),
                    loginCreds = it.loginCreds
                ).toReply()
            }
    }

    override fun getUser(request: StringValue): Uni<ProfileReply> = scope.asyncUni {
        searcher
            .getById(UUID.fromString(request.toString()))
            .awaitSuspending()
            .let {
                UserProfile(
                    name = it.name,
                    nickname = it.nickname,
                    gender = it.gender,
                    birth = it.birth,
                    gravatar = it.gravatar
                ).toReply()
            }
    }

    @WithTransaction
    override fun updateUser(request: ProfileRequest): Uni<ProfileReply> = scope.asyncUni {
        memberHandler
            .ask(
                id = UUID.fromString(request.id),
                cmd = MemberProfileChange(
                    nickname = request.nickname,
                    gender = if (request.hasGender()) Gender.valueOf(request.gender) else null,
                    birth = if (request.hasBirth()) LocalDate.parse(request.birth) else null,
                    gravatar = if (request.hasGravatar()) request.gravatar else null
                )
            )
            .awaitSuspending()
            .let {
                UserProfile(
                    name = it.name,
                    nickname = it.nickname,
                    gender = it.gender,
                    birth = it.birth,
                    gravatar = it.gravatar
                ).toReply()
            }
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<BoolValue> = scope.asyncUni {
        if (request.newPassword == request.confirm) {
            authenticationService
                .changePassword(UUID.fromString(request.id), request)
                .awaitSuspending()
                .let { BoolValue.of(true) }
        } else throw WebApplicationException("New password and confirmation password do not match", 400)
    }
}
