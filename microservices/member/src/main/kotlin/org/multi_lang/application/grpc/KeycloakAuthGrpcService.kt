package org.multi_lang.application.grpc

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
import org.multi_lang.application.event.publish.MemberCreatedEvent
import org.multi_lang.application.event.publish.MemberProducer
import org.multi_lang.domain.enums.IdentityMold
import org.multi_lang.domain.keycloak.KeyCloakTokenReply
import org.multi_lang.domain.message.ProcessReply
import org.multi_lang.infra.search.MemberSearcher
import org.multi_lang.domain.service.AuthenticationService
import org.multi_lang.domain.service.KeycloakService
import org.shared.utils.MnemonicUtil
import org.shared.utils.MyScope
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
        val user: org.multi_lang.domain.entity.Member? = authenticationService.checkMember(request.identifier).awaitSuspending()
        if (user != null) {
            ProcessReply(result = false, processedId = request.identifier).toResponse()
        } else keycloakService.check(request).awaitSuspending()
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val mnemonic = MnemonicUtil.generateMnemonic()

        val tokenResponse = keycloakService.register(request).awaitSuspending()
        if (tokenResponse.code == Status.OK.code.toString()) {

            val userId = keycloakService.getJwt(tokenResponse.data.accessToken).subject

            authenticationService.register(
                mold = IdentityMold.KeyCloak,
                loginCreds = request.loginCreds,
                password = mnemonic,
                userId = UUID.fromString(userId),
                nickname = request.nickname
            ).awaitSuspending()

            memberSearcher.getByName(request.loginCreds).awaitSuspending()?.run {
                // publish member create event to rabbitmq
                memberProducer.sendCreatedEvent(
                    MemberCreatedEvent(
                        user_id = this.userId,
                        user_name = this.name,
                        member_type = this.memberType,
                        member_id = this.id!!,
                        login_creds = this.loginCreds,
                        level = this.level,
                        my_referrer_code = this.referrerCode,
                        referee_code = request.refereeCode
                    )
                )
                getTokenResponse(this, tokenResponse.data)
            } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
        } else tokenResponse
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        keycloakService.changePassword(request).awaitSuspending()
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val tokenResponse = keycloakService.login(request)
        if (tokenResponse.code == Status.OK.code.toString()) {
            memberSearcher.getByName(request.identifier).awaitSuspending()?.run {
                getTokenResponse(this, tokenResponse.data)
            } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
        } else tokenResponse
    }

    private fun getTokenResponse(member: org.multi_lang.domain.entity.Member, token: KeycloakTokenResponse.KeycloakToken) =
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
