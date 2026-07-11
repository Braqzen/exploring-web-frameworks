from app.params import parse_id
from flask import current_app, jsonify, request
from pydantic import ValidationError
from app.state import AppState
from app.task import PatchedTask
from routes.errors import send_error, AppErrors


def patch_handler(id):
    try:
        task_id = parse_id(id)
    except ValidationError:
        return send_error(AppErrors.InvalidPath)

    body = request.get_json(silent=True)

    if body is None:
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task = PatchedTask.model_validate(body)
    except ValidationError:
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = current_app.extensions["state"]

    if task_id not in state.tasks:
        return send_error(AppErrors.TaskNotFound)

    state.tasks[task_id].operation = task.operation

    return jsonify(state.tasks[task_id].model_dump()), 200
