package org.acme.health

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured
import org.hamcrest.CoreMatchers
import org.junit.jupiter.api.Test


@QuarkusTest
class HealthCheckTest {
    @Test
    fun testHealthCheck() {
        RestAssured.given()
            .`when`()["/q/health/live"]
            .then()
            .statusCode(200)
            .body("status", CoreMatchers.`is`("UP"))
            .body("checks.size()", CoreMatchers.`is`(4))
            .body(
                "checks.name", CoreMatchers.everyItem(
                    CoreMatchers.anyOf(
                        CoreMatchers.`is`("Simple health check"),
                        CoreMatchers.`is`("Health check with data"),
                        CoreMatchers.`is`("alive"),
                        CoreMatchers.`is`("SmallRye Reactive Messaging - liveness check"),
                    )
                )
            )
            .body("checks.status", CoreMatchers.everyItem(CoreMatchers.`is`("UP")))
            .body("checks.data.foo[0]", CoreMatchers.`is`("fooValue"))
            .body("checks.data.bar[0]", CoreMatchers.`is`("barValue"))
    }
}
