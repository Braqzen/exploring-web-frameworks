import type { RouteHandler } from "fastify";
import { type State, parseTask, parseId } from "app";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";
import type { PutRoute } from "../types.js";

export function putHandler(state: State): RouteHandler<PutRoute> {
  return async (request, reply) => {
    const logger = getLogger();

    const id = parseId(request.params.id);
    if (!id.success) {
      logger.warn(
        { method: request.method, path: request.url },
        "Invalid path"
      );
      return sendError(reply, AppErrors.InvalidPath);
    }

    const task = parseTask(request.body);
    if (!task.success) {
      logger.warn(
        { method: request.method, path: request.url },
        "Invalid body JSON"
      );
      return sendError(reply, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn(
        { id: id.data, method: "PUT", path: request.url },
        "Task not found"
      );
      return sendError(reply, AppErrors.TaskNotFound);
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

    return reply.status(200).send(task.data);
  };
}
