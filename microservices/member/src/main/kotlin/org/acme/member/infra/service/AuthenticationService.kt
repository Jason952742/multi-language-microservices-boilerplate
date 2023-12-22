package org.acme.member.infra.service

import auth_proto.*
import common_proto.ProcessResponse
import io.grpc.Status
import org.acme.member.infra.repository.LoginPassesRepository
import org.acme.member.infra.repository.PasswordInfoRepository
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.acme.member.domain.entity.LoginPasses
import org.acme.member.domain.entity.Member
import org.acme.member.domain.entity.PasswordInfo
import org.acme.member.domain.enums.IdentityMold
import org.acme.member.domain.message.IdentityReply
import org.acme.member.domain.message.ProcessReply
import org.acme.member.infra.repository.MemberRepository
import org.acme.member.infra.search.MemberSearcher
import org.acme.utils.CaptchaUtils
import org.acme.utils.DateUtils
import org.acme.utils.EncryptionUtils.encrypt
import org.acme.utils.MutinyUtils.uniItem
import org.jboss.logging.Logger
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

    suspend fun checkMember(identifier: String): Uni<Member?> = memberRepository.findByName(identifier)

    suspend fun checkLoginPasses(mold: IdentityMold, identifier: String): Uni<LoginPasses?> = loginPassesRepository.findByIdentifier(mold, identifier)

    suspend fun authenticateCredentials(credentials: SignInRequest): Uni<IdentityResponse> = loginPassesRepository
        .findByIdentifier(IdentityMold.valueOf(credentials.mold), credentials.identifier)
        .awaitSuspending()?.let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, credentials.password).awaitSuspending()?.run {
                val identityReply: IdentityResponse = IdentityReply(userId = it.user.id!!, loginCreds = it.loginCreds).toResponse()
                uniItem(identityReply)
            } ?: uniItem(IdentityReply.toError(Status.UNAUTHENTICATED, "Incorrect user or password"))
        } ?: uniItem(IdentityReply.toError(Status.NOT_FOUND, "user not found"))

    suspend fun register(mold: IdentityMold, loginCreds: String, password: String, nickname: String?): Uni<IdentityResponse> = loginPassesRepository
        .findByLoginCreds(loginCreds)
        .awaitSuspending().run {
            when (this) {
                null -> {
                    val passwordInfo = PasswordInfo(
                        name = loginCreds,
                        loginCreds = loginCreds,
                        password = password.encrypt()
                    )
                    val member = Member(
                        name = passwordInfo.loginCreds,
                        nickname = nickname ?: "anonymous${CaptchaUtils.generator6Code()}",
                        loginCreds = passwordInfo.loginCreds,
                        passwordInfo = passwordInfo
                    )
                    val loginPasses = LoginPasses(
                        name = passwordInfo.name,
                        loginCreds = passwordInfo.loginCreds,
                        mold = mold,
                        identifier = loginCreds,
                        expired = DateUtils.farFarAway,
                        user = member
                    )

                    member.loginPasses.add(loginPasses)
                    val user = memberRepository.persist(member).awaitSuspending()
                    val identityReply: IdentityResponse = IdentityReply(userId = user.id!!, loginCreds = user.loginCreds).toResponse()
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
