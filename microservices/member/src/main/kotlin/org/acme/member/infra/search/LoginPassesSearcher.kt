package org.acme.member.infra.search

import org.acme.member.infra.repository.LoginPassesRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.acme.common.hibernate.JasFilter
import org.acme.common.hibernate.JasFilterOp
import org.acme.common.hibernate.JasQuery
import org.acme.common.resource.JasPaging
import org.acme.member.domain.entity.LoginPasses
import org.acme.member.domain.enums.IdentityMold
import org.acme.member.domain.message.LoginPassesListItem
import org.acme.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class LoginPassesSearcher {

    @Inject
    private lateinit var loginPassesRepository: LoginPassesRepository

    suspend fun getById(id: UUID): Uni<LoginPasses> = loginPassesRepository.get(id)

    suspend fun search(
        loginCreds: String?, mold: IdentityMold?, identifier: String?, name: String?,
        order_by: String?, order_asc: Boolean?, limit: Int, offset: Int
    ): Uni<JasPaging<LoginPassesListItem>> {
        val filters = listOf(
            JasFilter("loginCreds", JasFilterOp.Equal, loginCreds),
            JasFilter("mold", JasFilterOp.Equal, mold),
            JasFilter("identifier", JasFilterOp.Equal, identifier),
        )
        val q = JasQuery(filters, name, order_by, order_asc, limit, offset)
        return loginPassesRepository.searchAndCount(q)
    }

    suspend fun checkByName(name: String): Uni<Boolean> = uniItem(loginPassesRepository.findByName(name)?.let { true } ?: false )

}
