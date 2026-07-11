from app.params import parse_id
from flask import current_app, jsonify
from pydantic import ValidationError
from app.state import AppState
from routes.errors import send_error, AppErrors


def get_handler(id: str):
    try:
        task_id = parse_id(id)
    except ValidationError:
        return send_error(AppErrors.InvalidPath)

    state: AppState = current_app.extensions["state"]

    task = state.tasks.get(task_id)
    if task is None:
        return send_error(AppErrors.TaskNotFound)

    return jsonify(task.model_dump()), 200
