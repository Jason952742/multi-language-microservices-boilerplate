package org.multi_lang.infra.repository

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.enterprise.context.ApplicationScoped
import org.multi_lang.domain.entity.Member
import org.multi_lang.domain.entity.item.MemberListItem
import org.shared.common.hibernate.JasPanacheRepository
import org.shared.common.hibernate.JasQuery
import org.shared.common.resource.JasPaging
import org.shared.utils.MutinyUtils

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
