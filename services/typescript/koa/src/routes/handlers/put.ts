import type { RouterContext } from "@koa/router";
import { type State, parseTask, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function putHandler(state: State) {
  return async (ctx: RouterContext) => {
    const logger = getLogger();

    const id = parseId(ctx.params.id);
    if (!id.success) {
      logger.warn({ method: ctx.req.method, path: ctx.path }, "Invalid path");
      return sendError(ctx, AppErrors.InvalidPath);
    }

    const task = parseTask(ctx.request.body);
    if (!task.success) {
      logger.warn(
        { method: ctx.req.method, path: ctx.path },
        "Invalid body JSON"
      );
      return sendError(ctx, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn(
        { id: id.data, method: "PUT", path: ctx.path },
        "Task not found"
      );
      return sendError(ctx, AppErrors.TaskNotFound);
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

    ctx.status = 200;
    ctx.body = task.data;
  };
}
