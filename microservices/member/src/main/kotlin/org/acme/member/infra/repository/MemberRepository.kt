package org.acme.member.infra.repository

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.acme.common.hibernate.JasPanacheRepository
import org.acme.common.hibernate.JasQuery
import org.acme.common.resource.JasPaging
import org.acme.member.domain.entity.Member
import org.acme.member.domain.message.MemberListItem
import org.acme.utils.MutinyUtils

@ApplicationScoped
class MemberRepository : JasPanacheRepository<Member> {

    suspend fun searchAndCount(q: JasQuery): Uni<JasPaging<MemberListItem>> {
        val p = find(q.query(), q.sort(), q.params())
        val total = p
            .count()
            .awaitSuspending()
        val items = p.range<Member>(q.offset, q.last())
            .project(MemberListItem::class.java)
            .list<MemberListItem>()
            .awaitSuspending()
        return MutinyUtils.uniItem(JasPaging.of(q = q, items = items, total = total))
    }

}
