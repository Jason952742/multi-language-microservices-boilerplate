package org.demo

import examples.Greeter
import examples.GreeterGrpc
import examples.HelloReply
import examples.HelloRequest
import io.quarkus.grpc.GrpcClient
import io.smallrye.mutiny.Uni
import jakarta.ws.rs.GET
import jakarta.ws.rs.Path


@Path("/helloworld")
class HelloWorldEndpoint {

    @GrpcClient("hello")
    lateinit var blockingHelloService: GreeterGrpc.GreeterBlockingStub

    @GrpcClient("hello")
    lateinit var helloService: Greeter

    @GET
    @Path("/blocking/{name}")
    fun helloBlocking(name: String): String {
        val reply = blockingHelloService.sayHello(HelloRequest.newBuilder().setName(name).build())
        return generateResponse(reply)
    }

    @GET
    @Path("/mutiny/{name}")
    fun helloMutiny(name: String): Uni<String> {
        return helloService.sayHello(HelloRequest.newBuilder().setName(name).build())
            .onItem().transform { reply: HelloReply -> generateResponse(reply) }
    }

    fun generateResponse(reply: HelloReply): String {
        return String.format("%s! HelloWorldService has been called %d number of times.", reply.message, reply.count)
    }
}
