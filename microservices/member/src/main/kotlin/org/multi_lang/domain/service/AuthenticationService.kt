package org.multi_lang.domain.service

import auth_proto.*
import common_proto.ProcessResponse
import io.grpc.Status
import org.multi_lang.infra.repository.LoginPassesRepository
import org.multi_lang.infra.repository.PasswordInfoRepository
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.multi_lang.infra.repository.MemberRepository
import org.multi_lang.infra.search.MemberSearcher
import org.shared.utils.CaptchaUtils
import org.shared.utils.DateUtils
import org.shared.utils.EncryptionUtils.encrypt
import org.shared.utils.MutinyUtils.uniItem
import org.shared.utils.UuidUtils
import org.jboss.logging.Logger
import org.multi_lang.domain.enums.IdentityMold
import org.multi_lang.domain.message.IdentityReply
import org.multi_lang.domain.message.ProcessReply
import java.util.*

@ApplicationScoped
class AuthenticationService {

    @Inject
    private lateinit var loginPassesRepository: LoginPassesRepository

    @Inject
    private lateinit var passwordRepository: PasswordInfoRepository

    @Inject
    private lateinit var memberRepository: MemberRepository

    @Inject
    private lateinit var searcher: MemberSearcher

    suspend fun checkMember(identifier: String): Uni<org.multi_lang.domain.entity.Member?> = memberRepository.findByName(identifier)

    suspend fun checkLoginPasses(mold: IdentityMold, identifier: String): Uni<org.multi_lang.domain.entity.LoginPasses?> = loginPassesRepository.findByIdentifier(mold, identifier)

    suspend fun authenticateCredentials(credentials: SignInRequest): Uni<IdentityResponse> = loginPassesRepository
        .findByIdentifier(IdentityMold.valueOf(credentials.mold), credentials.identifier)
        .awaitSuspending()?.let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, credentials.password).awaitSuspending()?.run {
                // todo: get user profile
                val identityReply: IdentityResponse = IdentityReply(userId = it.user.id!!, loginCreds = it.loginCreds, nickname = "").toResponse()
                uniItem(identityReply)
            } ?: uniItem(IdentityReply.toError(Status.UNAUTHENTICATED, "Incorrect user or password"))
        } ?: uniItem(IdentityReply.toError(Status.NOT_FOUND, "user not found"))

    suspend fun register(mold: IdentityMold, loginCreds: String, password: String, userId: UUID, nickname: String?): Uni<IdentityResponse> = loginPassesRepository
        .findByLoginCreds(loginCreds)
        .awaitSuspending().run {
            when (this) {
                null -> {
                    val passwordInfo = org.multi_lang.domain.entity.PasswordInfo(
                        userId = userId,
                        name = loginCreds,
                        loginCreds = loginCreds,
                        password = password.encrypt()
                    )
                    val member = org.multi_lang.domain.entity.Member(
                        name = passwordInfo.loginCreds,
                        userId = userId,
                        nickname = if (nickname !== null && nickname !== "") nickname else "anonymous${CaptchaUtils.generator6Code()}",
                        loginCreds = passwordInfo.loginCreds,
                        passwordInfo = passwordInfo,
                        referrerCode = UuidUtils.encodeUUID(UUID.randomUUID())
                    )
                    val loginPasses = org.multi_lang.domain.entity.LoginPasses(
                        userId = userId,
                        name = passwordInfo.name,
                        loginCreds = passwordInfo.loginCreds,
                        mold = mold,
                        identifier = loginCreds,
                        expired = DateUtils.farFarAway,
                        user = member
                    )

                    member.loginPasses.add(loginPasses)
                    val user = memberRepository.persistAndFlush(member).awaitSuspending()
                    val identityReply: IdentityResponse = IdentityReply(userId = user.id!!, loginCreds = user.loginCreds, nickname = user.nickname).toResponse()
                    uniItem(identityReply)
                }

                else -> uniItem(IdentityReply.toError(Status.ALREADY_EXISTS, "user is already exist"))
            }
        }

    suspend fun changePassword(id: UUID, passwordChange: PasswordChangeRequest): Uni<ProcessResponse> = searcher
        .getById(id)
        .awaitSuspending().let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, passwordChange.oldPassword).awaitSuspending()?.run {
                val passwordInfo = passwordRepository.save(this.updateNewPassword(passwordChange.newPassword)).awaitSuspending()
                val processReply = ProcessReply(result = true, processedId = passwordInfo.loginCreds).toResponse()
                uniItem(processReply)
            } ?: uniItem(ProcessReply.toError(Status.UNAUTHENTICATED, "Old password incorrect"))
        }

    companion object {
        val LOG: Logger = Logger.getLogger(AuthenticationService::class.java.name)
        const val TITLE = "Auth Service"
    }
}
