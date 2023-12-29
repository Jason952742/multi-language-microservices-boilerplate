package org.multi_lang

import com.ecwid.consul.v1.ConsulClient
import com.ecwid.consul.v1.QueryParams
import com.ecwid.consul.v1.Response
import com.ecwid.consul.v1.agent.model.NewService
import com.ecwid.consul.v1.health.HealthServicesRequest
import com.ecwid.consul.v1.health.model.HealthService
import io.quarkus.scheduler.Scheduled
import jakarta.enterprise.context.ApplicationScoped
import org.shared.utils.StrUtils
import org.eclipse.microprofile.config.inject.ConfigProperty


@ApplicationScoped
class ConsulService {

    enum class ServiceName {
        MuReferral,
        MuMember;

        fun toSnakeCase(): String {
            return StrUtils.toSnakeCase(name)
        }

    }

    @ConfigProperty(name = "quarkus.grpc.server.port")
    private lateinit var grpcPort: String

    private val consulClient = ConsulClient("127.0.0.1")
    private val memberServiceName = ServiceName.MuMember.toSnakeCase()

    @Scheduled(every = "30s")
    fun discover() {
        discoverService(memberServiceName)
    }

    fun registerServiceWithHealthCheck() {
        val newService = NewService()
        newService.id = memberServiceName
        newService.name = memberServiceName
        newService.address = "localhost"
        newService.port = grpcPort.toInt()

        val serviceCheck = NewService.Check()
        serviceCheck.grpc="host.docker.internal:$grpcPort"
        serviceCheck.interval = "10s"
        newService.check = serviceCheck

        // register to Consul
        consulClient.agentServiceRegister(newService)
    }

    fun discoverService(serviceName: String) {
        val request = HealthServicesRequest.newBuilder()
            .setPassing(true)
            .setQueryParams(QueryParams.DEFAULT)
            .build()

        val healthyServices: Response<List<HealthService>> = consulClient.getHealthServices(serviceName, request)
        put(serviceName, healthyServices.value)
    }

    companion object {
        private val map: MutableMap<String, List<HealthService>> = mutableMapOf()

        fun put(key: String, value: List<HealthService>) {
            map[key] = value
        }

        fun get(key: String): List<HealthService>? {
            return map[key]
        }

        fun remove(key: String) {
            map.remove(key)
        }

        fun clear() {
            map.clear()
        }
    }
}
