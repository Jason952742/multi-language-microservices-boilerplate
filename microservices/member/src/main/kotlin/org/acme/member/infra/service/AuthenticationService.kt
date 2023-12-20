package org.acme.member.infra.service

import auth.PasswordChangeRequest
import auth.RegistrationRequest
import auth.SignInRequest
import org.acme.member.infra.repository.LoginPassesRepository
import org.acme.member.infra.repository.PasswordInfoRepository
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import org.acme.member.domain.entity.LoginPasses
import org.acme.member.domain.entity.Member
import org.acme.member.domain.entity.PasswordInfo
import org.acme.member.domain.entity.enums.IdentityMold
import org.acme.member.infra.repository.MemberRepository
import org.acme.member.infra.search.MemberSearcher
import org.acme.utils.DateUtils
import org.acme.utils.EncryptionUtils.encrypt
import org.acme.utils.MutinyUtils.uni
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

    suspend fun authenticateCredentials(credentials: SignInRequest): Uni<Member> = loginPassesRepository
        .findByIdentifier(IdentityMold.Phone, credentials.identifier)
        .awaitSuspending()?.let {
            // Verify that the password is correct
            passwordRepository.verify(it.loginCreds, credentials.password).awaitSuspending()?.run {
                return uni(it.user)
            } ?: throw WebApplicationException("Incorrect user or password", 401)
        } ?: throw WebApplicationException("user no regis", 401)

    suspend fun register(data: RegistrationRequest): Uni<Member> = loginPassesRepository
        .findByLoginCreds(data.loginCreds)
        .awaitSuspending().run {
            when(this) {
                null -> {
                    val passwordInfo = PasswordInfo(
                        name = data.loginCreds,
                        loginCreds = data.loginCreds,
                        password = data.password.encrypt()
                    )

                    val member = Member(
                        name = passwordInfo.loginCreds,
                        nickname = if (data.hasNickname()) data.nickname else "",
                        loginCreds = passwordInfo.loginCreds,
                        passwordInfo = passwordInfo
                    )

                    val loginPasses = LoginPasses(
                        name = passwordInfo.name,
                        loginCreds = passwordInfo.loginCreds,
                        mold = IdentityMold.valueOf(data.mold),
                        identifier = data.loginCreds,
                        expired = DateUtils.farFarAway,
                        user = member
                    )

                    member.loginPasses.add(loginPasses)
                    memberRepository.persist(member)
                }
                else -> uni(this.user)
            }
        }

    suspend fun changePassword(id: UUID, passwordChange: PasswordChangeRequest): Uni<PasswordInfo> {
        val member = searcher.getById(id).awaitSuspending()
        // Verify that the password is correct
        passwordRepository.verify(member.loginCreds, passwordChange.oldPassword).awaitSuspending()?.run {
            return passwordRepository.save(this.updateNewPassword(passwordChange.newPassword))
        } ?: throw WebApplicationException("Old password incorrect", 401)
    }

    companion object {
        val LOG: Logger = Logger.getLogger(AuthenticationService::class.java.name)
        const val title = "Auth Service"
    }
}
