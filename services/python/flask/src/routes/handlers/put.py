from app.params import parse_id
from flask import current_app, jsonify, request
from pydantic import ValidationError
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


def put_handler(id):
    try:
        task_id = parse_id(id)
    except ValidationError:
        return send_error(AppErrors.InvalidPath)

    body = request.get_json(silent=True)

    if body is None:
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task = Task.model_validate(body)
    except ValidationError:
        return send_error(AppErrors.InvalidJsonBody)

    state: AppState = current_app.extensions["state"]

    if task_id not in state.tasks:
        return send_error(AppErrors.TaskNotFound)

    state.tasks[task_id] = task

    return jsonify(task.model_dump()), 200
