package org.demo

import com.fasterxml.jackson.databind.ObjectMapper
import com.rabbitmq.client.*
import io.quarkus.test.junit.QuarkusTest
import org.awaitility.Awaitility
import org.demo.Quote
import org.eclipse.microprofile.config.inject.ConfigProperty
import org.hamcrest.Matchers
import org.junit.jupiter.api.Test
import java.nio.charset.StandardCharsets
import java.util.*
import java.util.Map
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicReference

@QuarkusTest
class QuoteProcessorTest {
    @ConfigProperty(name = "rabbitmq-host")
    lateinit var host: String

    @ConfigProperty(name = "rabbitmq-port")
    lateinit var port: String
    var objectMapper: ObjectMapper = ObjectMapper()

    @Test
    @Throws(Exception::class)
    fun testProcessor() {
        val quoteId = UUID.randomUUID().toString()

        val channel = channel

        channel.exchangeDeclare("quotes", BuiltinExchangeType.TOPIC, true, false, Map.of())

        val queue = channel.queueDeclare("quotes", true, false, false, Map.of())
            .queue
        channel.queueBind(queue, "quotes", "#")

        val receivedQuote = AtomicReference<Quote?>(null)

        val deliverCallback = DeliverCallback { consumerTag: String?, message: Delivery ->
            val quote = objectMapper.readValue(message.body, Quote::class.java)
            if (quote.id != quoteId) {
                return@DeliverCallback
            }
            receivedQuote.set(quote)
        }
        val consumerTag = channel.basicConsume(queue, true, deliverCallback) { tag: String? -> }

        val props = AMQP.BasicProperties.Builder()
            .contentType("text/plain")
            .build()
        channel.basicPublish("quote-requests", quoteId, props, quoteId.toByteArray(StandardCharsets.UTF_8))

        Awaitility.await().atMost(3, TimeUnit.SECONDS).untilAtomic(receivedQuote, Matchers.notNullValue())

        channel.basicCancel(consumerTag)
    }

    @get:Throws(Exception::class)
    val channel: Channel
        get() {
            val connectionFactory = ConnectionFactory()
            connectionFactory.host = host
            connectionFactory.port = port.toInt()
            connectionFactory.username = "rabbit"
            connectionFactory.password = "rabbitpassword"
            connectionFactory.channelRpcTimeout = TimeUnit.SECONDS.toMillis(3).toInt()

            val connection = connectionFactory.newConnection()
            return connection.createChannel()
        }
}
