import type { RouteHandler } from "fastify";
import { z } from "zod";
import type { State } from "../../state.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";
import type { DeleteRoute } from "../types.js";

export function deleteHandler(state: State): RouteHandler<DeleteRoute> {
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

    if (task === undefined) {
      logger.warn(
        { id: id.data, method: "DELETE", path: request.url },
        "Task not found"
      );
      return sendError(reply, AppErrors.TaskNotFound);
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

    return reply.status(204).send();
  };
}
