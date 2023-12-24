package org.acme

import com.ecwid.consul.v1.ConsulClient
import com.ecwid.consul.v1.agent.model.NewService
import jakarta.enterprise.context.ApplicationScoped
import org.eclipse.microprofile.config.inject.ConfigProperty

@ApplicationScoped
class ConsulService {

    @ConfigProperty(name = "quarkus.grpc.server.port")
    private lateinit var grpcPort: String

    private val consulClient = ConsulClient("127.0.0.1")

    fun registerServiceWithHealthCheck(serviceName: String) {
        val newService = NewService()
        newService.id = serviceName
        newService.name = serviceName
        newService.address = "localhost"
        newService.port = grpcPort.toInt()

        val serviceCheck = NewService.Check()
        serviceCheck.grpc="host.docker.internal:$grpcPort"
        serviceCheck.interval = "10s"
        newService.check = serviceCheck

        // register to Consul
        consulClient.agentServiceRegister(newService)
    }
}
