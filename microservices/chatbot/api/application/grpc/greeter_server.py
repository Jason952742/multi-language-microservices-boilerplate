import asyncio
import logging

import grpc
from api.proto.pb2 import helloworld_pb2_grpc, helloworld_pb2


class Greeter(helloworld_pb2_grpc.GreeterServicer):
    async def SayHello(
        self,
        request: helloworld_pb2.HelloRequest,
        context: grpc.aio.ServicerContext,
    ) -> helloworld_pb2.HelloReply:
        logging.info("Received request, sleeping for 4 seconds...")
        await asyncio.sleep(4)
        logging.info("Sleep completed, responding")
        return helloworld_pb2.HelloReply(message="Hello, %s!" % request.name)
