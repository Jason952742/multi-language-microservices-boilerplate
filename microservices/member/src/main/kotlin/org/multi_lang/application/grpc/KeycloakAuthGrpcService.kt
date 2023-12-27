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
import org.multi_lang.domain.message.MemberCreatedEvent
import org.multi_lang.application.event.publish.MemberProducer
import org.multi_lang.domain.entity.enums.IdentityMold
import org.multi_lang.application.grpc.assembler.KeyCloakTokenReply
import org.multi_lang.application.grpc.assembler.ProcessReply
import org.multi_lang.domain.entity.Member
import org.multi_lang.infra.search.MemberSearcher
import org.multi_lang.domain.service.AuthenticationService
import org.multi_lang.domain.service.KeycloakService
import org.multi_lang.infra.service.dto.KeycloakAccessToken
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
        val user: Member? = authenticationService.checkMember(request.identifier).awaitSuspending()
        if (user != null) {
            ProcessReply(result = false, processedId = request.identifier).toResponse()
        } else {
            keycloakService.check(request.identifier)
                .onItem().ifNotNull().transform {
                    ProcessReply(result = it.isEmpty(), processedId = request.identifier).toResponse()
                }.onFailure().transform { throwable ->
                    println("Received error: ${throwable.message}")
                    throwable
                }.awaitSuspending()
        }
    }

    @WithTransaction
    override fun register(request: RegistrationRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val mnemonic = MnemonicUtil.generateMnemonic()
        val token = keycloakService.register(request.loginCreds, request.password).awaitSuspending()

        val userId = keycloakService.getJwt(token.accessToken).subject

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
            getTokenResponse(this, token)
        } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
    }

    @WithTransaction
    override fun changePassword(request: PasswordChangeRequest): Uni<ProcessResponse> = scope.asyncUni {
        keycloakService.changePassword(UUID.fromString(request.id), request.newPassword).onItem().ifNotNull().transform {
            ProcessReply(result = true, processedId = request.id).toResponse()
        }.onFailure().transform { throwable ->
            println("Received error: ${throwable.message}")
            throwable
        }.awaitSuspending()
    }

    @WithSession
    override fun login(request: SignInRequest): Uni<KeycloakTokenResponse> = scope.asyncUni {
        val token = keycloakService.login(request.identifier, request.password).awaitSuspending()
        memberSearcher.getByName(request.identifier).awaitSuspending()?.run {
            val response = getTokenResponse(this, token)
            response
        } ?: KeyCloakTokenReply.toError(Status.NOT_FOUND, "member not found")
    }

    private fun getTokenResponse(member: Member, token: KeycloakAccessToken) =
        KeycloakTokenResponse.newBuilder().also {
            it.code = Status.OK.code.toString()
            it.message = "Success"
            it.data = token.toProto()
            it.userId = member.id.toString()
            it.userName = member.nickname
            it.referrerCode = member.referrerCode
            it.expiredAt = member.expiredAt.toString()
        }.build()

    private fun KeycloakAccessToken.toProto(): KeycloakTokenResponse.KeycloakToken = KeycloakTokenResponse.KeycloakToken.newBuilder().also {
        it.accessToken = accessToken
        it.expiresIn = expiresIn
        it.refreshExpiresIn = refreshExpiresIn
        it.refreshToken = refreshToken
        it.tokenType = tokenType
    }.build()

    private fun KeycloakAccessToken.toReply(): KeycloakTokenResponse = KeycloakTokenResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.data = this.toProto()
    }.build()

}
