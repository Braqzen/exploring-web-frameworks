from flask import current_app, jsonify, request
from pydantic import TypeAdapter, ValidationError
from pydantic.types import UUID4
from task import Task
from state import AppState


def put_handler(id):

    try:
        task_id = TypeAdapter(UUID4).validate_python(id)
    except ValidationError:
        return jsonify({"error": "Invalid path"}), 404

    body = request.get_json(silent=True)

    if body is None:
        return jsonify({"error": "Invalid body JSON"}), 422

    try:
        task = Task.model_validate(body)
    except ValidationError:
        return jsonify({"error": "Invalid body JSON"}), 422

    state: AppState = current_app.extensions["state"]

    if task_id not in state.tasks:
        return jsonify({"error": "Task not found"}), 404

    state.tasks[task_id] = task

    return jsonify(task.model_dump()), 200
