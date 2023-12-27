package org.multi_lang.infra.repository

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.multi_lang.domain.entity.LoginPasses
import org.multi_lang.domain.enums.IdentityMold
import org.multi_lang.domain.message.LoginPassesListItem
import org.shared.common.hibernate.JasPanacheRepository
import org.shared.common.hibernate.JasQuery
import org.shared.common.resource.JasPaging
import org.shared.utils.MutinyUtils

@ApplicationScoped
class LoginPassesRepository : JasPanacheRepository<LoginPasses> {

    fun findByLoginCreds(loginCreds: String): Uni<org.multi_lang.domain.entity.LoginPasses?> = find("loginCreds", loginCreds).firstResult()

    suspend fun findByIdentifier(mold: IdentityMold, identifier: String): Uni<org.multi_lang.domain.entity.LoginPasses?>  = find("mold = : mold and identifier = : identifier", mapOf(
        "mold" to mold,
        "identifier" to identifier
    )).firstResult()

    suspend fun searchAndCount(q: JasQuery): Uni<JasPaging<LoginPassesListItem>> {
        val p = find(q.query(), q.sort(), q.params())
        val total = p
            .count()
            .awaitSuspending()
        val items = p.range<org.multi_lang.domain.entity.LoginPasses>(q.offset, q.last())
            .project(LoginPassesListItem::class.java)
            .list<LoginPassesListItem>()
            .awaitSuspending()
        return MutinyUtils.uniItem(JasPaging.of(q = q, items = items, total = total))
    }

}
