package org.demo

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.util.*


@QuarkusTest
class QuotesResourceTest {
    @Test
    fun testQuotesEventStream() {
        val body = RestAssured.given()
            .`when`()
            .post("/quotes/request")
            .then()
            .statusCode(200)
            .extract().body()
            .asString()
        Assertions.assertDoesNotThrow<UUID> { UUID.fromString(body) }
    }
}
