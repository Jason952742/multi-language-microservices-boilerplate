package org.shared.common.resource

import io.vertx.core.http.HttpServerRequest
import jakarta.ws.rs.container.ContainerRequestContext
import jakarta.ws.rs.container.ContainerRequestFilter
import jakarta.ws.rs.core.Context
import jakarta.ws.rs.core.UriInfo
import jakarta.ws.rs.ext.Provider
import org.jboss.logging.Logger


@Provider
class JasLoggingFilter : ContainerRequestFilter {

    @Context
    lateinit var info: UriInfo

    @Context
    lateinit var request: HttpServerRequest

    override fun filter(context: ContainerRequestContext) {
        val method = context.method
        val path = info.path
        val address = request.remoteAddress().toString()
        LOG.infof("Request: %s %s from IP %s", method, path, address)
    }

    companion object {
        private val LOG = Logger.getLogger(JasLoggingFilter::class.java)
    }
}
