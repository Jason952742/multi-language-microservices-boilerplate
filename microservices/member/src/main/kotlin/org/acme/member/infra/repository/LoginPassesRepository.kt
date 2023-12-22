package org.acme.member.infra.repository

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.acme.common.hibernate.JasPanacheRepository
import org.acme.common.hibernate.JasQuery
import org.acme.common.resource.JasPaging
import org.acme.member.domain.entity.LoginPasses
import org.acme.member.domain.enums.IdentityMold
import org.acme.member.domain.message.LoginPassesListItem
import org.acme.utils.MutinyUtils

@ApplicationScoped
class LoginPassesRepository : JasPanacheRepository<LoginPasses> {

    fun findByLoginCreds(loginCreds: String): Uni<LoginPasses?> = find("loginCreds", loginCreds).firstResult()

    suspend fun findByIdentifier(mold: IdentityMold, identifier: String): Uni<LoginPasses?>  = find("mold = : mold and identifier = : identifier", mapOf(
        "mold" to mold,
        "identifier" to identifier
    )).firstResult()

    suspend fun searchAndCount(q: JasQuery): Uni<JasPaging<LoginPassesListItem>> {
        val p = find(q.query(), q.sort(), q.params())
        val total = p
            .count()
            .awaitSuspending()
        val items = p.range<LoginPasses>(q.offset, q.last())
            .project(LoginPassesListItem::class.java)
            .list<LoginPassesListItem>()
            .awaitSuspending()
        return MutinyUtils.uniItem(JasPaging.of(q = q, items = items, total = total))
    }

}
