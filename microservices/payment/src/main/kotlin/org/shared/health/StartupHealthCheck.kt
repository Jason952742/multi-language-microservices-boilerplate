package org.shared.health

import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.health.HealthCheck
import org.eclipse.microprofile.health.HealthCheckResponse
import org.eclipse.microprofile.health.Startup

@Startup
@ApplicationScoped
class StartupHealthCheck : HealthCheck {
    override fun call(): HealthCheckResponse {
        return HealthCheckResponse.up("Startup health check")
    }
}
