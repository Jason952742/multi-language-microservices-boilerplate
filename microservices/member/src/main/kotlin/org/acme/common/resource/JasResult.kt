package org.acme.common.resource

import jakarta.validation.constraints.NotNull
import kotlinx.serialization.Serializable


@Serializable
data class JasResult<T>(
    val code: ResultCode,
    val message: String? = null,
    val data: T? = null
) {

    companion object {
        private const val serialVersionUID = 1696194043024336235L


        fun <T> success(): JasResult<T> {
            return JasResult(code = ResultCode.SUCCESS)
        }


        fun <T> success(@NotNull data: T): JasResult<T> {
            return JasResult(code = ResultCode.SUCCESS, data = data)
        }

        fun error(message: String?): JasResult<Any> {
            return JasResult(code = ResultCode.FAILURE, message = message)
        }

        fun <T> error(message: String?, data: T): JasResult<T> {
            return JasResult(code = ResultCode.FAILURE, message = message, data = data)
        }
    }


    enum class ResultCode(val code: Int, val msg: String) {
        SUCCESS(1, "Call Succeeded"), // call succeeded
        FAILURE(-1, "System busy"); // The server is temporarily unavailable, it is recommended to try again later. It is recommended to retry no more than 3 times
    }
}
