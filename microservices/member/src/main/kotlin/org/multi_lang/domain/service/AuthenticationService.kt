package org.multi_lang.domain.service

import io.grpc.Status
import io.grpc.StatusException
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.jboss.logging.Logger
import org.multi_lang.domain.entity.LoginPasses
import org.multi_lang.domain.entity.Member
import org.multi_lang.domain.entity.PasswordInfo
import org.multi_lang.domain.entity.enums.IdentityMold
import org.multi_lang.infra.repository.LoginPassesRepository
import org.multi_lang.infra.repository.MemberRepository
import org.multi_lang.infra.repository.PasswordInfoRepository
import org.multi_lang.infra.search.MemberSearcher
import org.shared.utils.CaptchaUtils
import org.shared.utils.DateUtils
import org.shared.utils.EncryptionUtils.encrypt
import org.shared.utils.UuidUtils
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

    suspend fun authenticateCredentials(mold: IdentityMold, identifier: String, password: String): Uni<PasswordInfo> = loginPassesRepository
        .findByIdentifier(mold, identifier)
        .awaitSuspending()?.let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, password).awaitSuspending()?.run {
                Uni.createFrom().item(this)
            } ?: Uni.createFrom().failure(StatusException(Status.UNAUTHENTICATED.withDescription("Incorrect user or password")))
        } ?: Uni.createFrom().failure(StatusException(Status.NOT_FOUND.withDescription("user not found")))

    suspend fun register(mold: IdentityMold, loginCreds: String, password: String, userId: UUID, nickname: String?): Uni<Member> = loginPassesRepository
        .findByLoginCreds(loginCreds)
        .awaitSuspending().run {
            when (this) {
                null -> {
                    val passwordInfo = PasswordInfo(
                        userId = userId,
                        name = loginCreds,
                        loginCreds = loginCreds,
                        password = password.encrypt()
                    )
                    val member = Member(
                        name = passwordInfo.loginCreds,
                        userId = userId,
                        nickname = if (nickname !== null && nickname !== "") nickname else "anonymous${CaptchaUtils.generator6Code()}",
                        loginCreds = passwordInfo.loginCreds,
                        passwordInfo = passwordInfo,
                        referrerCode = UuidUtils.encodeUUID(UUID.randomUUID())
                    )
                    val loginPasses = LoginPasses(
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
                    Uni.createFrom().item(user)
                }
                else -> Uni.createFrom().failure(StatusException(Status.ALREADY_EXISTS.withDescription("user is already exist")))
            }
        }

    suspend fun changePassword(id: UUID, oldPassword: String, newPassword: String): Uni<PasswordInfo> = searcher
        .getById(id)
        .awaitSuspending().let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, oldPassword).awaitSuspending()?.run {
                val passwordInfo = passwordRepository.save(this.updateNewPassword(newPassword)).awaitSuspending()
                Uni.createFrom().item(passwordInfo)
            } ?: Uni.createFrom().failure(StatusException(Status.UNAUTHENTICATED.withDescription("Old password incorrect")))
        }

    companion object {
        val LOG: Logger = Logger.getLogger(AuthenticationService::class.java.name)
        const val TITLE = "Auth Service"
    }
}
