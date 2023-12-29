package org.multi_lang.domain.handler

import org.multi_lang.infra.repository.MemberRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import org.multi_lang.domain.command.*
import org.multi_lang.domain.entity.enums.MemberStatus.*
import org.shared.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class MemberHandler : org.shared.common.base.JasHandlerBase<org.multi_lang.domain.entity.Member, MemberCommand>() {

    @Inject
    @field: Default
    private lateinit var repo: MemberRepository

    override suspend fun ask(id: UUID, cmd: MemberCommand): Uni<org.multi_lang.domain.entity.Member> = entityRef(id, repo).let {
        when(entity.status) {
            Created -> when (cmd) {
                is MemberGet -> uniItem(it)
                is MemberProfileChange -> onUpdate(cmd)
                is MemberDelete -> delete(repo)
                else -> rejected(cmd)
            }
            InUse -> when (cmd) {
                is MemberGet -> uniItem(it)
                is MemberProfileChange -> onUpdate(cmd)
                else -> rejected(cmd)
            }
            Subscriber -> when (cmd) {
                is MemberGet -> uniItem(it)
                is MemberProfileChange -> onUpdate(cmd)
                else -> rejected(cmd)
            }
            Expired -> when (cmd) {
                is MemberGet -> uniItem(it)
                is MemberProfileChange -> onUpdate(cmd)
                else -> rejected(cmd)
            }
            Hide -> when (cmd) {
                is MemberDelete -> delete(repo)
                else -> rejected(cmd)
            }
            Disable -> when (cmd) {
                is MemberGet -> uniItem(it)
                is MemberDelete -> delete(repo)
                else -> rejected(cmd)
            }
        }
    }

    override suspend fun add(cmd: MemberCommand) = when (cmd) {
        is MemberCreate -> onCreate(cmd)
        else -> rejected(cmd)
    }

    private fun onCreate(cmd: MemberCreate): Uni<org.multi_lang.domain.entity.Member> = insert(cmd.member, repo)

    private fun onUpdate(cmd: MemberProfileChange): Uni<org.multi_lang.domain.entity.Member> = update(entity.apply {
        cmd.nickname?.let { nickname = it }
        description = cmd.description
    }, repo)

}
