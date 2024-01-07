from pydantic import BaseModel


class Message(BaseModel):
    """Simple message model."""

    Status: str
