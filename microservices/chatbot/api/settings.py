import enum
from pathlib import Path
from tempfile import gettempdir
from typing import Optional

from pydantic_settings import BaseSettings, SettingsConfigDict
from yarl import URL

TEMP_DIR = Path(gettempdir())


class LogLevel(str, enum.Enum):  # noqa: WPS600
    """Possible log levels."""

    NOTSET = "NOTSET"
    DEBUG = "DEBUG"
    INFO = "INFO"
    WARNING = "WARNING"
    ERROR = "ERROR"
    FATAL = "FATAL"


class Settings(BaseSettings):
    """
    Application settings.

    These parameters can be configured
    with environment variables.
    """

    host: str = "127.0.0.1"
    port: int = 8000
    # quantity of workers for uvicorn
    workers_count: int = 1
    # Enable uvicorn reloading
    reload: bool = False

    # Current environment
    environment: str = "dev"

    log_level: LogLevel = LogLevel.INFO

    # mongodb
    mongo_host: str = "localhost"
    mongo_port: int = 27017
    mongo_base: str = "chat_test"

    # Variables for Redis
    redis_host: str = "localhost"
    redis_port: int = 6379
    redis_user: Optional[str] = None
    redis_pass: Optional[str] = "mypass"
    redis_base: Optional[int] = None

    # Variables for RabbitMQ
    rabbit_host: str = "localhost"
    rabbit_port: int = 5672
    rabbit_user: str = "rabbit"
    rabbit_pass: str = "rabbitpassword"
    rabbit_vhost: str = "/"

    rabbit_pool_size: int = 2
    rabbit_channel_pool_size: int = 10

    # This variable is used to define
    # multiproc_dir. It's required for [uvi|guni]corn projects.
    prometheus_dir: Path = TEMP_DIR / "prom"

    @property
    def mongo_url(self) -> URL:
        """
        Assemble database URL from settings.

        :return: database URL.
        """
        return URL.build(
            scheme="mongodb",
            host=self.mongo_host,
            port=self.mongo_port,
            path=f"/{self.mongo_base}",
        )

    @property
    def redis_url(self) -> URL:
        """
        Assemble REDIS URL from settings.

        :return: redis URL.
        """
        path = ""
        if self.redis_base is not None:
            path = f"/{self.redis_base}"
        return URL.build(
            scheme="redis",
            host=self.redis_host,
            port=self.redis_port,
            user=self.redis_user,
            password=self.redis_pass,
            path=path,
        )

    @property
    def rabbit_url(self) -> URL:
        """
        Assemble RabbitMQ URL from settings.

        :return: rabbit URL.
        """
        return URL.build(
            scheme="amqp",
            host=self.rabbit_host,
            port=self.rabbit_port,
            user=self.rabbit_user,
            password=self.rabbit_pass,
            path=self.rabbit_vhost,
        )

    model_config = SettingsConfigDict(
        env_file=".env",
        env_prefix="API_",
        env_file_encoding="utf-8",
    )


settings = Settings()
