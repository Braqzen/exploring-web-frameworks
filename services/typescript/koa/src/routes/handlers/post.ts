import type { Context } from "koa";
import { type State, parseTask } from "app";
import { randomUUID } from "node:crypto";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function postHandler(state: State) {
  return async (ctx: Context) => {
    const logger = getLogger();

    const task = parseTask(ctx.request.body);
    if (!task.success) {
      logger.warn(
        { method: ctx.req.method, path: ctx.path },
        "Invalid body JSON"
      );
      return sendError(ctx, AppErrors.InvalidJsonBody);
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

    ctx.status = 201;
    ctx.body = { id };
  };
}
