from flask import current_app, jsonify
from pydantic import TypeAdapter, ValidationError
from pydantic.types import UUID4
from state import AppState


def get_handler(id):
    try:
        task_id = TypeAdapter(UUID4).validate_python(id)
    except ValidationError:
        return jsonify({"error": "Invalid path"}), 404

    state: AppState = current_app.extensions["state"]

    task = state.tasks.get(task_id)
    if task is None:
        return jsonify({"error": "Task not found"}), 404

    return jsonify(task.model_dump()), 200
