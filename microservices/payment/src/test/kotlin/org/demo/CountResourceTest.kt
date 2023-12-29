package org.demo

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured
import org.hamcrest.CoreMatchers
import org.junit.jupiter.api.Test


@QuarkusTest
class CountResourceTest {
    @Test
    fun testHelloEndpoint() {
        RestAssured.given()
            .`when`()["/count"]
            .then()
            .statusCode(200)
            .body(CoreMatchers.containsString("count"))
    }
}
