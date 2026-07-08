import type { RouterContext } from "@koa/router";
import { type State, parseId } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function deleteHandler(state: State) {
  return (ctx: RouterContext) => {
    const logger = getLogger();

    const id = parseId(ctx.params.id);
    if (!id.success) {
      logger.warn({ method: ctx.req.method, path: ctx.path }, "Invalid path");
      return sendError(ctx, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);

    if (task === undefined) {
      logger.warn(
        { id: id.data, method: "DELETE", path: ctx.path },
        "Task not found"
      );
      return sendError(ctx, AppErrors.TaskNotFound);
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

    ctx.status = 204;
    ctx.body = null;
  };
}
