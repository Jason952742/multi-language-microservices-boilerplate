package org.multi_lang

import io.quarkus.logging.LoggingFilter
import org.eclipse.microprofile.config.inject.ConfigProperty
import java.util.logging.Filter
import java.util.logging.LogRecord


@LoggingFilter(name = "my-filter")
class TestFilter(
    @param:ConfigProperty(name = "my-filter.legacy-gRPC") private val part: String
) : Filter {
    override fun isLoggable(record: LogRecord): Boolean {
        return !record.message.contains(part) && !record.message.contains("Installed features: [")
    }
}
