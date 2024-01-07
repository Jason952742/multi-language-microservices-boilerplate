from fastapi import APIRouter

from api.web.api.monitoring.schema import Message

router = APIRouter()


@router.get("/health", response_model=Message)
def health_check() -> Message:
    """
    Checks the health of a project.

    It returns 200 if the project is healthy.
    """
    # Check health status
    is_healthy = True  # Replace with your actual health check logic

    # Create JSON response
    status = "passing" if is_healthy else "failing"
    response = {"Status": status}

    return Message(Status="OK")
