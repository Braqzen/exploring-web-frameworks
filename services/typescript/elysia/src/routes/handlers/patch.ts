import type { Context } from "elysia";
import { type State, parseId, parsePatchedTask } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function patchHandler(state: State) {
  return ({ params, path, body, request, set }: Context) => {
    const logger = getLogger();

    const id = parseId(params.id);
    if (!id.success) {
      logger.warn({ method: request.method, path }, "Invalid path");
      return sendError(set, AppErrors.InvalidPath);
    }

    const patchedTask = parsePatchedTask(body);
    if (!patchedTask.success) {
      logger.warn({ method: request.method, path }, "Invalid body JSON");
      return sendError(set, AppErrors.InvalidJsonBody);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn({ id: id.data, method: "PATCH", path }, "Task not found");
      return sendError(set, AppErrors.TaskNotFound);
    }

    const previousOperation = task.operation;
    task.operation = patchedTask.data.operation;

    logger.info(
      {
        id: id.data,
        secret: task.secret.length,
        from_operation: previousOperation.toString().toLowerCase(),
        to_operation: task.operation.toString().toLowerCase(),
        method: "PATCH"
      },
      "Patched task"
    );

    set.status = 200;
    return task;
  };
}
