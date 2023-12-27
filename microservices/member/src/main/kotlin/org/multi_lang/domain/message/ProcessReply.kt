package org.multi_lang.domain.message

import common_proto.ProcessResponse
import io.grpc.Status

data class ProcessReply(val result: Boolean, val processedId: String) {

    fun toResponse(): ProcessResponse = ProcessResponse.newBuilder().also {
        it.code = Status.OK.code.toString()
        it.message = "Success"
        it.result = result
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
