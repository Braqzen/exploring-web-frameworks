import type { Context } from "hono";
import { randomUUID } from "node:crypto";
import { type State } from "../../state.js";
import { Task } from "../../task.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function postHandler(state: State) {
  return async (c: Context) => {
    const logger = getLogger();

    const body = await c.req.json();
    let task = Task.safeParse(body);
    if (!task.success) {
      logger.warn(
        { method: c.req.method, path: c.req.path },
        "Invalid body JSON"
      );
      return sendError(c, AppErrors.InvalidJsonBody);
    }

    let id = randomUUID();

    state.tasks.set(id, task.data);

    logger.info(
      {
        id,
        secret: task.data.secret.length,
        operation: task.data.operation.toString().toLowerCase(),
        method: "POST"
      },
      "Inserted new task"
    );

    return c.json({ id }, 201);
  };
}
