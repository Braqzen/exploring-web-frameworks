import type { Context } from "elysia";
import { type State, parseTask, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function putHandler(state: State) {
  return ({ params, path, body, request, set }: Context) => {
    const logger = getLogger();

    const id = parseId(params.id);
    if (!id.success) {
      logger.warn({ method: request.method, path }, "Invalid path");
      return sendError(set, AppErrors.InvalidPath);
    }

    const task = parseTask(body);
    if (!task.success) {
      logger.warn({ method: request.method, path }, "Invalid body JSON");
      return sendError(set, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn({ id: id.data, method: "PUT", path }, "Task not found");
      return sendError(set, AppErrors.TaskNotFound);
    }

    state.tasks.set(id.data, task.data);

    logger.info(
      {
        id: id.data,
        from_secret: previous_task.secret.length,
        to_secret: task.data.secret.length,
        from_operation: previous_task.operation.toString().toLowerCase(),
        to_operation: task.data.operation.toString().toLowerCase(),
        method: "PUT"
      },
      "Overwrote task"
    );

    set.status = 200;
    return task.data;
  };
}
