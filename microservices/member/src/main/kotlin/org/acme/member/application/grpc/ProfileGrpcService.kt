package org.acme.member.application.grpc

import com.google.protobuf.BoolValue
import org.acme.member.infra.search.MemberSearcher
import org.acme.member.infra.service.PasswordService
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.common.model.Gender
import org.acme.member.domain.commands.SystemUserProfileChange
import org.acme.member.domain.entity.valobj.UserProfile
import org.acme.member.domain.handler.MemberHandler
import org.acme.utils.MyScope
import profile.PasswordChangeRequest
import profile.ProfileReply
import profile.ProfileRequest
import profile.ProfileServer
import java.time.LocalDate
import java.util.*

@GrpcService
@ExperimentalCoroutinesApi
class ProfileGrpcService : ProfileServer {

    @Inject
    lateinit var scope: MyScope

    @Inject
    @field: Default
    lateinit var service: PasswordService

    @Inject
    lateinit var searcher: MemberSearcher

    @Inject
    lateinit var memberHandler: MemberHandler

    override fun getUser(id: utils.UUID): Uni<ProfileReply> = scope.asyncUni {
        searcher
            .getById(UUID.fromString(id.value.toString()))
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
    override fun update(request: ProfileRequest): Uni<ProfileReply> = scope.asyncUni {
        memberHandler
            .ask(
                id = UUID.fromString(request.id.value),
                cmd = SystemUserProfileChange(
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
            service
                .changePassword(UUID.fromString(request.id.value), request)
                .awaitSuspending()
                .let { BoolValue.of(true) }
        } else throw WebApplicationException("New password and confirmation password do not match", 400)
    }
}
