from flask import current_app, jsonify, request
from uuid import uuid4
from pydantic import ValidationError
from task import Task
from state import AppState


def post_handler():

    body = request.get_json(silent=True)

    if body is None:
        return jsonify({"error": "Invalid body JSON"}), 422

    try:
        task = Task.model_validate(body)
    except ValidationError:
        return jsonify({"error": "Invalid body JSON"}), 422

    id = uuid4()

    state: AppState = current_app.extensions["state"]

    state.tasks[id] = task

    return jsonify({"id": str(id)}), 201
