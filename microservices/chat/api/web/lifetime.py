from typing import Awaitable, Callable

from fastapi import FastAPI
from prometheus_fastapi_instrumentator.instrumentation import (
    PrometheusFastApiInstrumentator,
)

from api.services.rabbit.lifetime import init_rabbit, shutdown_rabbit
from api.services.redis.lifetime import init_redis, shutdown_redis

import consul


def register_consul(service_name, ip, port):
    c = consul.Consul()
    service_id = f"{service_name}-{port}"
    service_url = f"http://{ip}:{port}/health"

    result = c.agent.service.register(
        name=service_name,
        service_id=service_id,
        address=ip,
        port=port,
        check=consul.Check.http(url=service_url, interval="10s")
    )
    if result:
        print("Service registration successful")
    else:
        print("Service registration failed")


def unregister_consul(service_id):
    c = consul.Consul()
    result = c.agent.service.deregister(service_id=service_id)
    if result:
        print(f"Service deregister {service_id} successful")
    else:
        print(f"Service deregister {service_id} failed")


def setup_prometheus(app: FastAPI) -> None:  # pragma: no cover
    """
    Enables prometheus integration.

    :param app: current application.
    """
    PrometheusFastApiInstrumentator(should_group_status_codes=False).instrument(
        app,
    ).expose(app, should_gzip=True, name="prometheus_metrics")


def register_startup_event(
    app: FastAPI,
) -> Callable[[], Awaitable[None]]:  # pragma: no cover
    """
    Actions to run on application startup.

    This function uses fastAPI app to store data
    in the state, such as db_engine.

    :param app: the fastAPI application.
    :return: function that actually performs actions.
    """

    @app.on_event("startup")
    async def _startup() -> None:  # noqa: WPS430
        app.middleware_stack = None
        init_redis(app)
        init_rabbit(app)
        setup_prometheus(app)
        app.middleware_stack = app.build_middleware_stack()

        register_consul("MuChat", "host.docker.internal", 50036)

        pass  # noqa: WPS420

    return _startup


def register_shutdown_event(
    app: FastAPI,
) -> Callable[[], Awaitable[None]]:  # pragma: no cover
    """
    Actions to run on application's shutdown.

    :param app: fastAPI application.
    :return: function that actually performs actions.
    """

    @app.on_event("shutdown")
    async def _shutdown() -> None:  # noqa: WPS430
        await shutdown_redis(app)
        await shutdown_rabbit(app)

        unregister_consul("MuChat-50036")
        pass  # noqa: WPS420

    return _shutdown
