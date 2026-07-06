import type { RouteHandler } from "fastify";
import { z } from "zod";
import type { State } from "../../state.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";
import type { GetRoute } from "../types.js";

export function getHandler(state: State): RouteHandler<GetRoute> {
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

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "GET", path: request.url },
        "Task not found"
      );
      return sendError(reply, AppErrors.TaskNotFound);
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

    return reply.status(200).send(task);
  };
}
