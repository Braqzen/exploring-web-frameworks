import type { Context } from "hono";
import { type State, parseTask } from "app";
import { randomUUID } from "node:crypto";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function postHandler(state: State) {
  return async (c: Context) => {
    const logger = getLogger();

    const body = await c.req.json();
    let task = parseTask(body);
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
