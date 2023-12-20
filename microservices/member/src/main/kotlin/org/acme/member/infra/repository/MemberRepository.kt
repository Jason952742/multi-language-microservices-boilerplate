package org.acme.member.infra.repository

import org.acme.member.domain.model.SystemUserListItem
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.acme.common.hibernate.JasPanacheRepository
import org.acme.common.hibernate.JasQuery
import org.acme.common.resource.JasPaging
import org.acme.member.domain.entity.Member
import org.acme.utils.MutinyUtils

@ApplicationScoped
class MemberRepository : JasPanacheRepository<Member> {

    suspend fun searchAndCount(q: JasQuery): Uni<JasPaging<SystemUserListItem>> {
        val p = find(q.query(), q.sort(), q.params())
        val total = p
            .count()
            .awaitSuspending()
        val items = p.range<Member>(q.offset, q.last())
            .project(SystemUserListItem::class.java)
            .list<SystemUserListItem>()
            .awaitSuspending()
        return MutinyUtils.uniItem(JasPaging.of(q = q, items = items, total = total))
    }

}
