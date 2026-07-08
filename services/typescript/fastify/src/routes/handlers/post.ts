import type { RouteHandler } from "fastify";
import { randomUUID } from "node:crypto";
import { type State, parseTask } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";
import type { PostRoute } from "../types.js";

export function postHandler(state: State): RouteHandler<PostRoute> {
  return async (request, reply) => {
    const logger = getLogger();

    let task = parseTask(request.body);
    if (!task.success) {
      logger.warn(
        { method: request.method, path: request.url },
        "Invalid body JSON"
      );
      return sendError(reply, AppErrors.InvalidJsonBody);
    }

    let id = randomUUID();

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

    return reply.status(201).send({ id });
  };
}
