package org.demo

import io.quarkus.test.junit.QuarkusTest
import jakarta.inject.Inject
import org.demo.GreetingService
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test


@QuarkusTest
class GreetingServiceTest {
    @Inject
    lateinit var service: GreetingService

    @Test
    fun testGreetingService() {
        val greeting = service.greeting("Quarkus").await()
        Assertions.assertEquals("hello Quarkus", greeting.indefinitely())
    }
}
