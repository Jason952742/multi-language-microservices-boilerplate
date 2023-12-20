package org.acme.member.domain.message

import auth.ProcessResponse
import io.grpc.Status

data class ProcessReply(
    var changed: Boolean,
    var processedId: String,
) {

    fun toResponse(): ProcessResponse = ProcessResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.changed = this.changed
        it.processedId = processedId
    }.build()

    companion object {
        fun toError(status: Status, message: String): ProcessResponse = ProcessResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
            it.changed = false
        }.build()
    }
}
