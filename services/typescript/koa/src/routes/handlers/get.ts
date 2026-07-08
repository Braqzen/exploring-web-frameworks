import type { RouterContext } from "@koa/router";
import { type State, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function getHandler(state: State) {
  return (ctx: RouterContext) => {
    const logger = getLogger();

    const id = parseId(ctx.params.id);
    if (!id.success) {
      logger.warn({ method: ctx.req.method, path: ctx.path }, "Invalid path");
      return sendError(ctx, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "GET", path: ctx.path },
        "Task not found"
      );
      return sendError(ctx, AppErrors.TaskNotFound);
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

    ctx.status = 200;
    ctx.body = task;
  };
}
