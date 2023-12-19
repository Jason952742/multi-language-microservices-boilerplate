package org.acme.demo

import io.quarkus.test.common.http.TestHTTPResource
import io.quarkus.test.junit.QuarkusTest
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.io.IOException
import java.net.URL
import java.nio.charset.StandardCharsets


@QuarkusTest
class StaticContentTest {
    @TestHTTPResource("index.html")
    lateinit var url: URL

    @Test
    @Throws(IOException::class)
    fun testIndexHtml() {
        url.openStream().use { `in` ->
            val contents = String(`in`.readAllBytes(), StandardCharsets.UTF_8)
            Assertions.assertTrue(contents.contains("<title>Testing Guide</title>"))
        }
    }
}
