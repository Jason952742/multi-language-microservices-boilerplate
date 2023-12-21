package org.acme.member.domain.message

import common_proto.ProcessResponse
import io.grpc.Status

data class ProcessReply(val changed: Boolean, val processedId: String) {

    fun toResponse(): ProcessResponse = ProcessResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.result = this.changed
        it.processedId = processedId
    }.build()

    companion object {
        fun toError(status: Status, message: String): ProcessResponse = ProcessResponse.newBuilder().also {
            it.code = status.code.toString()
            it.message = message
            it.result = false
        }.build()
    }
}
