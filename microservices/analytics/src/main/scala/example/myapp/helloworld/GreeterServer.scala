package example.myapp.helloworld

import akka.actor.ActorSystem
import akka.http.scaladsl.model.{HttpRequest, HttpResponse}
import akka.http.scaladsl.Http
import com.ecwid.consul.v1.ConsulClient
import com.ecwid.consul.v1.agent.model.NewService
import com.typesafe.config.ConfigFactory
import example.myapp.helloworld.grpc.*

import java.util
import scala.concurrent.{ExecutionContext, Future}

object GreeterServer {
  def main(args: Array[String]): Unit = {
    // Important: enable HTTP/2 in ActorSystem's config
    // We do it here programmatically, but you can also set it in the application.conf
    val conf =
      ConfigFactory.parseString("akka.http.server.enable-http2 = on").withFallback(ConfigFactory.defaultApplication())
    val system = ActorSystem("HelloWorld", conf)
    new GreeterServer(system).run()
    // ActorSystem threads will keep the app alive until `system.terminate()` is called
  }
}

class GreeterServer(system: ActorSystem) {
  private def run(): Future[Http.ServerBinding] = {
    // Akka boot up code
    implicit val sys: ActorSystem = system
    implicit val ec: ExecutionContext = sys.dispatcher

    // Create service handlers
    val service: HttpRequest => Future[HttpResponse] =
      GreeterServiceHandler(new GreeterServiceImpl())

    // Bind service handler servers to localhost:8080/8081
    val binding = Http().newServerAt("127.0.0.1", 50036).bind(service)

    register()

    // report successful binding
    binding.foreach { binding => println(s"gRPC server bound to: ${binding.localAddress}") }

    binding
  }

  private def register(): Unit = {
    val client = new ConsulClient("localhost")
    // register new service// register new service

    val newService = new NewService()
    newService.setId("myapp_01")
    newService.setName("myapp")
    newService.setTags(util.Arrays.asList("EU-West", "EU-East"))
    newService.setPort(50036)

    val serviceCheck = new NewService.Check()
    serviceCheck.setGrpc("localhost:50036")
    serviceCheck.setInterval("10s")
    newService.setCheck(serviceCheck)

    client.agentServiceRegister(newService)

  }
}