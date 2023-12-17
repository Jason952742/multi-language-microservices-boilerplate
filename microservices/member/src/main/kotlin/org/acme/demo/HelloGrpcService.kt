package org.acme.demo

import io.quarkus.grpc.GrpcService
import io.smallrye.mutiny.Uni
import java.util.concurrent.atomic.AtomicInteger

@GrpcService
class HelloWorldService : Greeter {
    private var counter: AtomicInteger = AtomicInteger()

    override fun sayHello(request: HelloRequest): Uni<HelloReply> {
        val count = counter.incrementAndGet()
        val name = request.name
        return Uni.createFrom().item("Hello $name")
            .map { res: String? -> HelloReply.newBuilder().setMessage(res).setCount(count).build() }
    }
}
