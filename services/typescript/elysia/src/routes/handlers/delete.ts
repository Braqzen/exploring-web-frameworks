import type { Context } from "elysia";
import { type State, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function deleteHandler(state: State) {
  return ({ params, path, request, set }: Context) => {
    const logger = getLogger();

    const id = parseId(params.id);
    if (!id.success) {
      logger.warn({ method: request.method, path }, "Invalid path");
      return sendError(set, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);

    if (task === undefined) {
      logger.warn({ id: id.data, method: "DELETE", path }, "Task not found");
      return sendError(set, AppErrors.TaskNotFound);
    }

    state.tasks.delete(id.data);

    logger.info(
      {
        id: id.data,
        secret: task.secret.length,
        operation: task.operation.toString().toLowerCase(),
        method: "DELETE"
      },
      "Removed task"
    );

    set.status = 204;
    return null;
  };
}
