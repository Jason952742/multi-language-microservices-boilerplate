package org.acme.member.domain.handler

import org.acme.member.infra.repository.MemberRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import org.acme.common.base.JasHandlerBase
import org.acme.member.domain.entity.Member
import org.acme.member.domain.enums.MemberStatus.*
import org.acme.member.domain.message.*
import org.acme.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class MemberHandler : JasHandlerBase<Member, MemberCommand>() {

    @Inject
    @field: Default
    private lateinit var repo: MemberRepository

    override suspend fun ask(id: UUID, cmd: MemberCommand): Uni<Member> = entityRef(id, repo).let {
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

    private fun onCreate(cmd: MemberCreate): Uni<Member> = insert(cmd.member, repo)

    private fun onUpdate(cmd: MemberProfileChange): Uni<Member> = update(entity.apply {
        cmd.nickname?.let { nickname = it }
        description = cmd.description
    }, repo)

}
