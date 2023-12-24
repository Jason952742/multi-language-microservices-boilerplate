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
import org.acme.common.model.SystemDefault
import org.acme.member.application.event.publish.MemberCreatedEvent
import org.acme.member.application.event.publish.MemberProducer
import org.acme.member.domain.entity.Member
import org.acme.member.domain.enums.IdentityMold
import org.acme.member.domain.keycloak.KeyCloakTokenReply
import org.acme.member.domain.message.ProcessReply
import org.acme.member.infra.search.MemberSearcher
import org.acme.member.infra.service.AuthenticationService
import org.acme.member.infra.service.KeycloakService
import org.acme.utils.MnemonicUtil
import org.acme.utils.MyScope
import java.util.*


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

    @Inject
    @field: Default
    lateinit var memberSearcher: MemberSearcher

    @Inject
    lateinit var memberProducer: MemberProducer

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

            val userId = keycloakService.getJwt(token.data.accessToken).subject

            authenticationService.register(
                mold = IdentityMold.KeyCloak,
                loginCreds = request.loginCreds,
                password = mnemonic,
                userId = UUID.fromString(userId),
                nickname = request.nickname
            ).awaitSuspending()

            memberSearcher.getByName(request.loginCreds).awaitSuspending()?.run {

                memberProducer.sendCreatedEvent(
                    MemberCreatedEvent(
                        userId = this.userId,
                        userName = this.name,
                        memberType = this.memberType,
                        memberId = this.id!!,
                        loginCreds = this.loginCreds,
                        level = this.level,
                        memberReferrerCode = this.referrerCode,
                        refereeId = SystemDefault.ID,
                        refereeName = SystemDefault.NAME,
                        refereeReferrerCode = SystemDefault.NAME
                    )
                )

                getTokenResponse(this, token.data)
            } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
        } else {
            KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
        }
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        keycloakService.changePassword(request).awaitSuspending()
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val token = keycloakService.login(request)
        if (token.code == Status.OK.code.toString()) {
            memberSearcher.getByName(request.identifier).awaitSuspending()?.run {
                getTokenResponse(this, token.data)
            } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
        } else {
            KeyCloakTokenReply.toError(Status.ALREADY_EXISTS, "member not found")
        }
    }

    private fun getTokenResponse(member: Member, token: KeycloakTokenResponse.KeycloakToken) =
        KeycloakTokenResponse.newBuilder().also {
            it.code = Status.OK.code.toString()
            it.message = "Success"
            it.data = token
            it.userId = member.id.toString()
            it.userName = member.nickname
            it.referrerCode = member.referrerCode
            it.expiredAt = member.expiredAt.toString()
        }.build()

}
