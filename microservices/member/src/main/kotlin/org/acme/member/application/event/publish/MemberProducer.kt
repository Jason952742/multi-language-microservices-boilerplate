package org.acme.member.application.event.publish

import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.reactive.messaging.Channel
import org.eclipse.microprofile.reactive.messaging.Emitter


@ApplicationScoped
class MemberProducer {

    @Channel("member-created")
    private lateinit var createdEmitter: Emitter<MemberCreatedEvent>

    fun sendCreatedEvent(event: MemberCreatedEvent) : MemberCreatedEvent {
        createdEmitter.send(event)
        println("A producer quote request has been sent to member-created, id $event")
        return event
    }

}
