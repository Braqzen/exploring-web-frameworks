import type { Context } from "hono";
import { type State, parseTask, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function putHandler(state: State) {
  return async (c: Context) => {
    const logger = getLogger();

    const id = parseId(c.req.param("id"));
    if (!id.success) {
      logger.warn({ method: c.req.method, path: c.req.path }, "Invalid path");
      return sendError(c, AppErrors.InvalidPath);
    }

    const body = await c.req.json();
    const task = parseTask(body);
    if (!task.success) {
      logger.warn(
        { method: c.req.method, path: c.req.path },
        "Invalid body JSON"
      );
      return sendError(c, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn(
        { id: id.data, method: "PUT", path: c.req.path },
        "Task not found"
      );
      return sendError(c, AppErrors.TaskNotFound);
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

    return c.json(task.data, 200);
  };
}
