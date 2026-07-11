from flask import current_app, jsonify, request
from uuid import uuid4
from pydantic import ValidationError
from app.task import Task
from app.state import AppState
from routes.errors import send_error, AppErrors


def post_handler():
    body = request.get_json(silent=True)

    if body is None:
        return send_error(AppErrors.InvalidJsonBody)

    try:
        task = Task.model_validate(body)
    except ValidationError:
        return send_error(AppErrors.InvalidJsonBody)

    id = uuid4()

    state: AppState = current_app.extensions["state"]

    state.tasks[id] = task

    return jsonify({"id": str(id)}), 201
