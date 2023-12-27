package org.multi_lang.infra.search

import org.multi_lang.infra.repository.MemberRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.multi_lang.domain.entity.item.MemberListItem
import org.shared.common.hibernate.JasFilter
import org.shared.common.hibernate.JasFilterOp
import org.shared.common.hibernate.JasQuery
import org.shared.common.resource.JasPaging
import org.shared.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class MemberSearcher {

    @Inject
    private lateinit var memberRepository: MemberRepository

    suspend fun getById(id: UUID): Uni<org.multi_lang.domain.entity.Member> = memberRepository.get(id)

    suspend fun getByName(name: String): Uni<org.multi_lang.domain.entity.Member?> = memberRepository.findByName(name)

    suspend fun search(
        nickname: String?, id_card: String?, name: String?,
        order_by: String?, order_asc: Boolean?, limit: Int, offset: Int
    ): Uni<JasPaging<MemberListItem>> {
        val filters = listOf(
            JasFilter("nickname", JasFilterOp.Like, nickname),
            JasFilter("id_card", JasFilterOp.Equal, id_card),
        )
        val q = JasQuery(filters, name, order_by, order_asc, limit, offset)
        return memberRepository.searchAndCount(q)
    }

    suspend fun checkByName(name: String): Uni<Boolean> = uniItem(memberRepository.findByName(name)?.let { true } ?: false)

}
