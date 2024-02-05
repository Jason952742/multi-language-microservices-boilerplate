package api.application.grpc

import akka.NotUsed
import akka.stream.Materializer
import akka.stream.scaladsl.Source
import io.grpc.health.v1.*
import io.grpc.health.v1.HealthCheckResponse.ServingStatus

import scala.concurrent.Future

class HealthServiceImpl(implicit mat: Materializer) extends Health {

  override def check(in: HealthCheckRequest): Future[HealthCheckResponse] = {
    Future.successful(HealthCheckResponse(status = ServingStatus.SERVING))
  }

  override def watch(in: HealthCheckRequest): Source[HealthCheckResponse, NotUsed] = {
    Source.single(HealthCheckResponse(status = ServingStatus.SERVING))
  }

}