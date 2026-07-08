import type { Context } from "hono";
import { type State, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function getHandler(state: State) {
  return (c: Context) => {
    const logger = getLogger();

    const id = parseId(c.req.param("id"));
    if (!id.success) {
      logger.warn({ method: c.req.method, path: c.req.path }, "Invalid path");
      return sendError(c, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "GET", path: c.req.path },
        "Task not found"
      );
      return sendError(c, AppErrors.TaskNotFound);
    }

    logger.info(
      {
        id: id.data,
        secret: task.secret.length,
        operation: task.operation.toString().toLowerCase(),
        method: "GET"
      },
      "Retrieved task"
    );

    return c.json(task, 200);
  };
}
