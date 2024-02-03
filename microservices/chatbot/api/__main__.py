import os
import shutil
from typing import Callable, Awaitable

import grpc

from api import lifetime
from api.proto.pb2 import helloworld_pb2_grpc, health_pb2_grpc

from api.application.grpc import greeter_server, health_server
from api.infra.settings import settings
import asyncio
import logging


def set_multiproc_dir() -> None:
    """
    Sets mutiproc_dir env variable.

    This function cleans up the multiprocess directory
    and recreates it. These actions are required by prometheus-client
    to share metrics between processes.

    After cleanup, it sets two variables.
    Uppercase and lowercase because different
    versions of the prometheus-client library
    depend on different environment variables,
    so I've decided to export all needed variables,
    to avoid undefined behaviour.
    """
    shutil.rmtree(settings.prometheus_dir, ignore_errors=True)
    os.makedirs(settings.prometheus_dir, exist_ok=True)
    os.environ["prometheus_multiproc_dir"] = str(
        settings.prometheus_dir.expanduser().absolute(),
    )
    os.environ["PROMETHEUS_MULTIPROC_DIR"] = str(
        settings.prometheus_dir.expanduser().absolute(),
    )


# Coroutines to be invoked when the event loop is shutting down.
_cleanup_coroutines = []

_AUTH_HEADER_KEY = "authorization"
_AUTH_HEADER_VALUE = "Bearer example_oauth2_token"


class SignatureValidationInterceptor(grpc.aio.ServerInterceptor):
    def __init__(self):
        def abort(ignored_request,  context: grpc.aio.ServicerContext):
            context.abort(grpc.StatusCode.UNAUTHENTICATED, "Invalid signature")

        self._abort_handler = grpc.unary_unary_rpc_method_handler(abort)

    async def intercept_service(
        self,
        continuation: Callable[
            [grpc.HandlerCallDetails], Awaitable[grpc.RpcMethodHandler]
        ],
        handler_call_details: grpc.HandlerCallDetails,
    ) -> grpc.RpcMethodHandler:
        # Example HandlerCallDetails object:
        #     _HandlerCallDetails(
        #       method=u'/helloworld.Greeter/SayHello',
        #       invocation_metadata=...)
        expected_metadata = (_AUTH_HEADER_KEY, _AUTH_HEADER_VALUE)
        if expected_metadata in handler_call_details.invocation_metadata:
            return await continuation(handler_call_details)
        else:
            return self._abort_handler


async def serve() -> None:
    server = grpc.aio.server(interceptors=(SignatureValidationInterceptor(),))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(greeter_server.Greeter(), server)
    health_pb2_grpc.add_HealthServicer_to_server(health_server.HealthServicer(), server)

    listen_addr = f"[::]:{settings.port}"
    server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)
    await lifetime.startup()
    await server.start()

    async def server_graceful_shutdown():
        logging.info("Starting graceful shutdown...")
        # Shuts down the server with 5 seconds of grace period. During the
        # grace period, the server won't accept new connections and allow
        # existing RPCs to continue within the grace period.
        await lifetime.shutdown()
        await server.stop(5)

    _cleanup_coroutines.append(server_graceful_shutdown())
    await server.wait_for_termination()


def main() -> None:
    """Entrypoint of the application."""
    set_multiproc_dir()

    # start grpc service
    logging.basicConfig(level=logging.INFO)
    loop = asyncio.get_event_loop()
    try:
        loop.run_until_complete(serve())
    finally:
        loop.run_until_complete(*_cleanup_coroutines)
        loop.close()


if __name__ == "__main__":
    main()
