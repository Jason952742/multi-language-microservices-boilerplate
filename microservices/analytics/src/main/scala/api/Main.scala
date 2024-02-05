package api

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import akka.http.scaladsl.model.{HttpRequest, HttpResponse}
import akka.http.scaladsl.server.{AuthorizationFailedRejection, Directive0, Directives}
import akka.http.scaladsl.server.Directives.*
import api.application.grpc.{GreeterServiceImpl, HealthServiceImpl}
import com.orbitz.consul.Consul
import com.orbitz.consul.model.agent.{ImmutableRegistration, Registration}
import com.typesafe.config.ConfigFactory
import example.myapp.helloworld.grpc.*

import scala.concurrent.{ExecutionContext, Future}

object Main {
  def main(args: Array[String]): Unit = {
    // Important: enable HTTP/2 in ActorSystem's config
    // We do it here programmatically, but you can also set it in the application.conf
    val conf =
      ConfigFactory.parseString("akka.http.server.enable-http2 = on").withFallback(ConfigFactory.defaultApplication())
    val system = ActorSystem("HelloWorld", conf)
    new Main(system).run()
    // ActorSystem threads will keep the app alive until `system.terminate()` is called
  }
}

class Main(system: ActorSystem) {
  private def run(): Future[Http.ServerBinding] = {
    // Akka boot up code
    implicit val sys: ActorSystem = system
    implicit val ec: ExecutionContext = sys.dispatcher

    val greeterService: PartialFunction[HttpRequest, Future[HttpResponse]] =
      example.myapp.helloworld.grpc.GreeterServiceHandler.partial(new GreeterServiceImpl())

    val greeterRoute = handle(greeterService)
    val authorizationDirective: Directive0 =
      headerValueByName("authorization").flatMap { token =>
        if (token == "Bearer yourtoken") pass
        else reject(AuthorizationFailedRejection)
      }

    val healthService: PartialFunction[HttpRequest, Future[HttpResponse]] =
      io.grpc.health.v1.HealthHandler.partial(new HealthServiceImpl())

    val healthRoute = handle(healthService)

    val route = Directives.concat(
      healthRoute,
      authorizationDirective {
        greeterRoute
      })

    // Bind service handler servers to localhost:8080/8081
    val binding = Http().newServerAt("127.0.0.1", 50036).bind(route)

    register()

    // report successful binding
    binding.foreach { binding => println(s"gRPC server bound to: ${binding.localAddress}") }

    binding
  }

  private def register(): Unit = {
    val client = Consul.builder.build
    val agentClient = client.agentClient

    val name = "MuChat"
    val port = 50036

    val serviceId = f"$name-$port"
    val service = ImmutableRegistration.builder().id(serviceId).name(name).port(port).check(
      Registration.RegCheck.grpc(f"host.docker.internal:$port", 10)
    ).build()

    agentClient.register(service)
  }
}