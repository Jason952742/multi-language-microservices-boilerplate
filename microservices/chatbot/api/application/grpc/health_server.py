from api.proto.pb2 import health_pb2_grpc, health_pb2


class HealthServicer(health_pb2_grpc.HealthServicer):
    def Check(self, request, context):
        response = health_pb2.HealthCheckResponse()
        # health status
        response.status = health_pb2.HealthCheckResponse.SERVING
        return response
