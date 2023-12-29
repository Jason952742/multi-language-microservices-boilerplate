package org.demo

import io.quarkus.test.junit.QuarkusTest
import io.restassured.RestAssured
import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test


@QuarkusTest
internal class HelloWorldEndpointTest {
    @Test
    fun testHelloWorldServiceUsingBlockingStub() {
        val response = RestAssured.get("/helloworld/blocking/neo").asString()
        Assertions.assertThat(response).startsWith("Hello neo")
    }

    @Test
    fun testHelloWorldServiceUsingMutinyStub() {
        val response = RestAssured.get("/helloworld/mutiny/neo-mutiny").asString()
        Assertions.assertThat(response).startsWith("Hello neo-mutiny")
    }
}
