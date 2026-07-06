import type { RouteHandler } from "fastify";
import { z } from "zod";
import type { State } from "../../state.js";
import { PatchedTask } from "../../task.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";
import type { PatchRoute } from "../types.js";

export function patchHandler(state: State): RouteHandler<PatchRoute> {
  return async (request, reply) => {
    const logger = getLogger();

    const id = z.uuidv4().safeParse(request.params.id);
    if (!id.success) {
      logger.warn(
        { method: request.method, path: request.url },
        "Invalid path"
      );
      return sendError(reply, AppErrors.InvalidPath);
    }

    const patchedTask = PatchedTask.safeParse(request.body);
    if (!patchedTask.success) {
      logger.warn(
        { method: request.method, path: request.url },
        "Invalid body JSON"
      );
      return sendError(reply, AppErrors.InvalidJsonBody);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "PATCH", path: request.url },
        "Task not found"
      );
      return sendError(reply, AppErrors.TaskNotFound);
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

    return reply.status(200).send(task);
  };
}
