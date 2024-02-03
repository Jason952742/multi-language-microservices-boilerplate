from api.infra.consul import register_consul, unregister_consul
from api.infra.settings import settings


async def startup() -> None:  # noqa: WPS430
    register_consul("MuChat", "host.docker.internal", settings.port)

    pass  # noqa: WPS420


async def shutdown() -> None:  # noqa: WPS430

    unregister_consul(f"MuChat-{settings.port}")
    pass  # noqa: WPS420
