package org.acme.member.infra.search

import org.acme.member.infra.repository.MemberRepository
import org.acme.member.domain.model.SystemUserListItem
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.acme.common.hibernate.JasFilter
import org.acme.common.hibernate.JasFilterOp
import org.acme.common.hibernate.JasQuery
import org.acme.common.resource.JasPaging
import org.acme.member.domain.entity.Member
import org.acme.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class MemberSearcher {

    @Inject
    private lateinit var memberRepository: MemberRepository

    suspend fun getById(id: UUID): Uni<Member> = memberRepository.get(id)

    suspend fun search(
        nickname: String?, id_card: String?, name: String?,
        order_by: String?, order_asc: Boolean?, limit: Int, offset: Int
    ): Uni<JasPaging<SystemUserListItem>> {
        val filters = listOf(
            JasFilter("nickname", JasFilterOp.Like, nickname),
            JasFilter("id_card", JasFilterOp.Equal, id_card),
        )
        val q = JasQuery(filters, name, order_by, order_asc, limit, offset)
        return memberRepository.searchAndCount(q)
    }

    suspend fun checkByName(name: String): Uni<Boolean> = uniItem(memberRepository.findByName(name)?.let { true } ?: false)

}
