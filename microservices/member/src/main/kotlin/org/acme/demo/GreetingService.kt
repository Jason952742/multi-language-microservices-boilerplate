package org.acme.demo

import io.smallrye.mutiny.Uni
import io.smallrye.mutiny.infrastructure.Infrastructure
import jakarta.enterprise.context.ApplicationScoped

@ApplicationScoped
class GreetingService {
    fun greeting(name: String): Uni<String> {
        return Uni.createFrom().item("hello $name")
            .emitOn(Infrastructure.getDefaultExecutor())
    }
}
