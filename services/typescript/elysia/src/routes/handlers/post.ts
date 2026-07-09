import type { Context } from "elysia";
import { type State, parseTask } from "app";
import { randomUUID } from "node:crypto";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function postHandler(state: State) {
  return ({ path, request, body, set }: Context) => {
    const logger = getLogger();

    const task = parseTask(body);
    if (!task.success) {
      logger.warn({ method: request.method, path }, "Invalid body JSON");
      return sendError(set, AppErrors.InvalidJsonBody);
    }

    const id = randomUUID();

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

    set.status = 201;
    return { id };
  };
}
