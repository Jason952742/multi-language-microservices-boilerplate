package api.infra.grpc

import akka.actor.ActorSystem
import akka.grpc.GrpcClientSettings
import akka.stream.scaladsl.Source
import akka.{Done, NotUsed}
import example.myapp.helloworld.grpc.*

import scala.concurrent.duration.*
import scala.concurrent.{ExecutionContextExecutor, Future}
import scala.util.{Failure, Success}

object GreeterClient {
  def main(args: Array[String]): Unit = {
    // Boot akka
    implicit val sys: ActorSystem = ActorSystem("HelloWorldClient")
    implicit val ec: ExecutionContextExecutor = sys.dispatcher

    // Configure the client by code:
    val clientSettings = GrpcClientSettings.connectToServiceAt("127.0.0.1", 50036).withTls(false)

    // Or via application.conf:
    // val clientSettings = GrpcClientSettings.fromConfig(GreeterService.name)

    // Create a client-side stub for the service
    val client: GreeterService = GreeterServiceClient(clientSettings)

    // Run examples for each of the exposed service methods.
    runSingleRequestReplyExample()
    runStreamingRequestExample()
    runStreamingReplyExample()
    runStreamingRequestReplyExample()

    sys.scheduler.scheduleWithFixedDelay(1.second, 1.second) { () => runSingleRequestReplyExample() }

    def runSingleRequestReplyExample(): Unit = {
      sys.log.info("Performing request")
      val reply = client.sayHello(HelloRequest("Alice"))
      reply.onComplete {
        case Success(msg) =>
          println(s"got single reply: $msg")
        case Failure(e) =>
          println(s"Error sayHello: $e")
      }
    }

    def runStreamingRequestExample(): Unit = {
      val requests = List("Alice", "Bob", "Peter").map(HelloRequest(_))
      val reply = client.itKeepsTalking(Source(requests))
      reply.onComplete {
        case Success(msg) =>
          println(s"got single reply for streaming requests: $msg")
        case Failure(e) =>
          println(s"Error streamingRequest: $e")
      }
    }

    def runStreamingReplyExample(): Unit = {
      val responseStream = client.itKeepsReplying(HelloRequest("Alice"))
      val done: Future[Done] =
        responseStream.runForeach(reply => println(s"got streaming reply: ${reply.message}"))

      done.onComplete {
        case Success(_) =>
          println("streamingReply done")
        case Failure(e) =>
          println(s"Error streamingReply: $e")
      }
    }

    def runStreamingRequestReplyExample(): Unit = {
      val requestStream: Source[HelloRequest, NotUsed] =
        Source
          .tick(100.millis, 1.second, "tick")
          .zipWithIndex
          .map { case (_, i) => i }
          .map(i => HelloRequest(s"Alice-$i"))
          .take(10)
          .mapMaterializedValue(_ => NotUsed)

      val responseStream: Source[HelloReply, NotUsed] = client.streamHellos(requestStream)
      val done: Future[Done] =
        responseStream.runForeach(reply => println(s"got streaming reply: ${reply.message}"))

      done.onComplete {
        case Success(_) =>
          println("streamingRequestReply done")
        case Failure(e) =>
          println(s"Error streamingRequestReply: $e")
      }
    }
  }
}