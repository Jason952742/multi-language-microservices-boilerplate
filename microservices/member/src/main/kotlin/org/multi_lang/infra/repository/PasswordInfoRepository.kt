package org.multi_lang.infra.repository

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.multi_lang.domain.entity.PasswordInfo
import org.shared.common.hibernate.JasPanacheRepository
import org.shared.common.hibernate.JasQuery
import org.shared.common.resource.JasPaging
import org.shared.utils.EncryptionUtils.encrypt
import org.shared.utils.MutinyUtils

@ApplicationScoped
class PasswordInfoRepository : JasPanacheRepository<PasswordInfo> {

    fun save(model: PasswordInfo): Uni<PasswordInfo> = persist(model.apply { password.encrypt() })

    fun verify(loginCreds: String, password: String): Uni<PasswordInfo?> = find(
        "loginCreds = :loginCreds and password = :password",
        mapOf("loginCreds" to loginCreds, "password" to password.encrypt())
    ).firstResult()

    fun findByLoginCreds(loginCreds: String): Uni<PasswordInfo?> = find("loginCreds", loginCreds).firstResult()

    suspend fun searchAndCount(q: JasQuery): Uni<JasPaging<PasswordInfo>> {
        val p = find(q.query(), q.sort(), q.params())
        val total = p
            .count()
            .awaitSuspending()
        val items = p.range<PasswordInfo>(q.offset, q.last())
            .list<PasswordInfo>()
            .awaitSuspending()
        return MutinyUtils.uniItem(JasPaging.of(q = q, items = items, total = total))
    }

}
