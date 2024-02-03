import os
import shutil

from api.application.grpc import greeter_server
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


def main() -> None:
    """Entrypoint of the application."""
    set_multiproc_dir()

    # start grpc service
    print("grpc service start...")
    logging.basicConfig(level=logging.INFO)
    asyncio.run(greeter_server.serve())


if __name__ == "__main__":
    main()
