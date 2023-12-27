package org.multi_lang.infra.search

import org.multi_lang.infra.repository.LoginPassesRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.multi_lang.domain.enums.IdentityMold
import org.multi_lang.domain.message.LoginPassesListItem
import org.shared.common.hibernate.JasFilter
import org.shared.common.hibernate.JasFilterOp
import org.shared.common.hibernate.JasQuery
import org.shared.common.resource.JasPaging
import org.shared.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class LoginPassesSearcher {

    @Inject
    private lateinit var loginPassesRepository: LoginPassesRepository

    suspend fun getById(id: UUID): Uni<org.multi_lang.domain.entity.LoginPasses> = loginPassesRepository.get(id)

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
