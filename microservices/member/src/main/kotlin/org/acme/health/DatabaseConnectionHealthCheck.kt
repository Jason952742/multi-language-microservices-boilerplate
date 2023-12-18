package org.acme.health

import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.config.inject.ConfigProperty
import org.eclipse.microprofile.health.HealthCheck
import org.eclipse.microprofile.health.HealthCheckResponse
import org.eclipse.microprofile.health.Readiness

@Readiness
@ApplicationScoped
class DatabaseConnectionHealthCheck : HealthCheck {

    @ConfigProperty(name = "database.up", defaultValue = "false")
    private lateinit var databaseUp: String

    override fun call(): HealthCheckResponse {
        val responseBuilder = HealthCheckResponse.named("Database connection health check")

        try {
            simulateDatabaseConnectionVerification()
            responseBuilder.up()
        } catch (e: IllegalStateException) {
            // cannot access the database
            responseBuilder.down()
                .withData("error", e.message) // pass the exception message
        }

        return responseBuilder.build()
    }

    private fun simulateDatabaseConnectionVerification() {
        check(databaseUp.toBoolean()) { "Cannot contact database" }
    }
}
