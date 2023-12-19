package org.acme.common.base

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.node.ObjectNode
import io.smallrye.mutiny.CompositeException
import io.smallrye.mutiny.Uni
import jakarta.inject.Inject
import jakarta.ws.rs.WebApplicationException
import jakarta.ws.rs.core.Response
import jakarta.ws.rs.core.Response.Status.*
import jakarta.ws.rs.ext.ExceptionMapper
import jakarta.ws.rs.ext.Provider
import kotlinx.coroutines.ExperimentalCoroutinesApi
import org.acme.common.resource.JasResult
import org.acme.utils.MyScope
import org.jboss.logging.Logger

@ExperimentalCoroutinesApi
open class JasResourceBase {

    @Inject
    lateinit var scope: MyScope

    fun <T> created(data: T): Response = Response.ok(JasResult.success(data)).status(CREATED).build()
    fun <T> ok(data: T): JasResult<T> = JasResult.success(data)
    fun accepted(): Response = Response.status(ACCEPTED).build()
    fun notFound(): Response = Response.status(NOT_FOUND).build()
    fun notContent(): Response = Response.status(NO_CONTENT).build()
    fun error(message: String?): JasResult<Any> = JasResult.error(message)
    fun error(code: Int, message: String?): Response {
        return Response.status(code).entity(JasResult.error(message)).build()
    }

    fun <T> error(message: String?, data: T): Response {
        return Response.status(500).entity(JasResult.error(message, data)).build()
    }

    fun <T> ok(data: Uni<T>): Uni<JasResult<T>> = data.onItem().transform { JasResult.success(it) }

    /**
     * Create a HTTP response from an exception.
     *
     * Response Example:
     *
     * <pre>
     * HTTP/1.1 422 Unprocessable Entity
     * Content-Length: 111
     * Content-Type: application/json
     *
     * {
     * "code": 422,
     * "error": "Fruit name was not set on request.",
     * "exceptionType": "jakarta.ws.rs.WebApplicationException"
     * }
    </pre> *
     */
    @Provider
    class ErrorMapper : ExceptionMapper<Exception> {
        @Inject
        lateinit var objectMapper: ObjectMapper

        override fun toResponse(exception: Exception): Response {
            LOG.error("Failed to handle request", exception)

            var throwable: Throwable = exception

            var code = 500
            if (throwable is WebApplicationException) {
                code = (exception as WebApplicationException).response.status
            }

            // This is a Mutiny exception and it happens, for example, when we try to insert a new
            // fruit but the name is already in the database
            if (throwable is CompositeException) {
                throwable = (throwable).cause!!
            }

            val exceptionJson: ObjectNode = objectMapper.createObjectNode()
            exceptionJson.put("exceptionType", throwable.javaClass.name)
            exceptionJson.put("code", code)

            if (exception.message != null) {
                exceptionJson.put("error", throwable.message)
            }

            return Response.status(code)
                .entity(JasResult.error("Failed to handle request", exceptionJson))
                .build()
        }
    }

    companion object {
        private val LOG = Logger.getLogger(JasResourceBase::class.java.name)
    }
}
