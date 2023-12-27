package org.multi_lang.application.event.publish

import io.smallrye.reactive.messaging.rabbitmq.OutgoingRabbitMQMetadata
import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.reactive.messaging.Channel
import org.eclipse.microprofile.reactive.messaging.Emitter
import org.eclipse.microprofile.reactive.messaging.Message
import org.eclipse.microprofile.reactive.messaging.Metadata
import java.time.ZonedDateTime

@ApplicationScoped
class MemberProducer {

    @Channel("member")
    private lateinit var createdEmitter: Emitter<MemberCreatedEvent>

    fun sendCreatedEvent(event: MemberCreatedEvent) : Message<MemberCreatedEvent>? {
        val metadata = OutgoingRabbitMQMetadata.Builder()
            .withHeader("my-header", "xyzzy")
            .withRoutingKey("created")
            .withTimestamp(ZonedDateTime.now())
            .build()

        // Add `metadata` to the metadata of the outgoing message.
        val message = Message.of(event, Metadata.of(metadata))

        createdEmitter.send(message)
        println("A producer quote request has been sent to member-created, id $event")
        return message
    }

}
