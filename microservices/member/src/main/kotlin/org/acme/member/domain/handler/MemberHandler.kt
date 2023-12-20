package org.acme.member.domain.handler

import org.acme.member.infra.repository.MemberRepository
import io.smallrye.mutiny.Uni
import jakarta.enterprise.context.ApplicationScoped
import jakarta.enterprise.inject.Default
import jakarta.inject.Inject
import org.acme.common.base.JasHandlerBase
import org.acme.member.domain.entity.Member
import org.acme.member.domain.message.*
import org.acme.utils.MutinyUtils.uniItem
import java.util.*

@ApplicationScoped
class MemberHandler : JasHandlerBase<Member, MemberCommand>() {

    @Inject
    @field: Default
    private lateinit var repo: MemberRepository

    override suspend fun ask(id: UUID, cmd: MemberCommand): Uni<Member> = entityRef(id, repo).let {
        when (cmd) {
            is MemberGet -> uniItem(it)
            is MemberProfileChange -> onUpdate(cmd)
            is MemberDelete -> delete(repo)
            else -> rejected(cmd)
        }
    }

    override suspend fun add(cmd: MemberCommand) = when (cmd) {
        is MemberCreate -> onCreate(cmd)
        else -> rejected(cmd)
    }

    private fun onCreate(cmd: MemberCreate): Uni<Member> {
        return insert(cmd.member, repo)
    }

    private fun onUpdate(cmd: MemberProfileChange): Uni<Member> = update(entity.apply {
        println("准备修改")
        println(cmd)

        cmd.nickname?.let { nickname = it }
        cmd.gender?.let { gender = it }
        cmd.birth?.let { birth = it }
        cmd.gravatar?.let { gravatar = it }
    }, repo)

}
