import type { Context } from "hono";
import { z } from "zod";
import type { State } from "../../state.js";
import { PatchedTask } from "../../task.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function patchHandler(state: State) {
  return async (c: Context) => {
    const logger = getLogger();

    const id = z.uuidv4().safeParse(c.req.param("id"));
    if (!id.success) {
      logger.warn({ method: c.req.method, path: c.req.path }, "Invalid path");
      return sendError(c, AppErrors.InvalidPath);
    }

    const body = await c.req.json();
    const patchedTask = PatchedTask.safeParse(body);
    if (!patchedTask.success) {
      logger.warn(
        { method: c.req.method, path: c.req.path },
        "Invalid body JSON"
      );
      return sendError(c, AppErrors.InvalidJsonBody);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "PATCH", path: c.req.path },
        "Task not found"
      );
      return sendError(c, AppErrors.TaskNotFound);
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

    return c.json(task, 200);
  };
}
