package org.multi_lang.application.grpc

import member_proto.*
import com.google.protobuf.StringValue
import common_proto.ProcessResponse
import io.quarkus.grpc.GrpcService
import io.quarkus.hibernate.reactive.panache.common.WithSession
import io.quarkus.hibernate.reactive.panache.common.WithTransaction
import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.coroutines.awaitSuspending
import jakarta.inject.Inject
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.multi_lang.domain.message.MemberProfileChange
import org.multi_lang.domain.handler.MemberHandler
import org.multi_lang.domain.message.MemberDelete
import org.multi_lang.domain.message.MemberReply
import org.multi_lang.domain.message.ProcessReply
import org.multi_lang.infra.search.MemberSearcher
import org.shared.utils.MyScope
import java.util.*

@GrpcService
@ExperimentalCoroutinesApi
class MemberGrpcService : MemberProtoService {

    @Inject
    lateinit var scope: MyScope

    @Inject
    lateinit var searcher: MemberSearcher

    @Inject
    lateinit var memberHandler: MemberHandler

    @WithSession
    override fun getMember(request: StringValue): Uni<MemberResponse> = scope.asyncUni {
        searcher.getById(UUID.fromString(request.value)).awaitSuspending().let {
            MemberReply(
                name = it.name,
                nickname = it.nickname,
                status = it.status,
                memberType = it.memberType,
                description = it.description
            ).toResponse()
        }
    }

    @WithTransaction
    override fun updateMember(request: MemberUpdateRequest): Uni<MemberResponse> = scope.asyncUni {
        val cmd = MemberProfileChange.fromProto(request)
        memberHandler.ask(id = UUID.fromString(request.id), cmd = cmd).awaitSuspending().let {
            MemberReply(
                name = it.name,
                nickname = it.nickname,
                status = it.status,
                memberType = it.memberType,
                description = it.description
            ).toResponse()
        }
    }

    @WithTransaction
    override fun deleteMember(request: StringValue): Uni<ProcessResponse> = scope.asyncUni {
        val id = UUID.fromString(request.value)
        memberHandler.ask(id = id, cmd = MemberDelete()).awaitSuspending().let {
            ProcessReply(result = true, processedId = id.toString()).toResponse()
        }
    }
}
