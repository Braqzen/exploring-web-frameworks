import type { RouterContext } from "@koa/router";
import { type State, parseId, parsePatchedTask } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function patchHandler(state: State) {
  return async (ctx: RouterContext) => {
    const logger = getLogger();

    const id = parseId(ctx.params.id);
    if (!id.success) {
      logger.warn({ method: ctx.req.method, path: ctx.path }, "Invalid path");
      return sendError(ctx, AppErrors.InvalidPath);
    }

    const patchedTask = parsePatchedTask(ctx.request.body);
    if (!patchedTask.success) {
      logger.warn(
        { method: ctx.req.method, path: ctx.path },
        "Invalid body JSON"
      );
      return sendError(ctx, AppErrors.InvalidJsonBody);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "PATCH", path: ctx.path },
        "Task not found"
      );
      return sendError(ctx, AppErrors.TaskNotFound);
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

    ctx.status = 200;
    ctx.body = task;
  };
}
