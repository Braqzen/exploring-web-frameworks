import type { Context } from "hono";
import { z } from "zod";
import type { State } from "../../state.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function deleteHandler(state: State) {
  return (c: Context) => {
    const logger = getLogger();

    const id = z.uuidv4().safeParse(c.req.param("id"));
    if (!id.success) {
      logger.warn({ method: c.req.method, path: c.req.path }, "Invalid path");
      return sendError(c, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);

    if (task === undefined) {
      logger.warn(
        { id: id.data, method: "DELETE", path: c.req.path },
        "Task not found"
      );
      return sendError(c, AppErrors.TaskNotFound);
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

    return c.body(null, 204);
  };
}
