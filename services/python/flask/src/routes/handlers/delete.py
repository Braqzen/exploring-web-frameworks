from flask import current_app
from pydantic import ValidationError
from app.state import AppState
from app.params import parse_id
from routes.errors import send_error, AppErrors


def delete_handler(id: str):
    try:
        task_id = parse_id(id)
    except ValidationError:
        return send_error(AppErrors.InvalidPath)

    state: AppState = current_app.extensions["state"]

    if state.tasks.pop(task_id, None) is None:
        return send_error(AppErrors.TaskNotFound)

    return "", 204
