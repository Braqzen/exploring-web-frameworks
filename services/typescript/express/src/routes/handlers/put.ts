import type { Request, RequestHandler, Response } from "express";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";
import { parseId, parseTask, type State } from "app";

export function putHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    const id = parseId(req.params.id);
    if (!id.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid path");
      return sendError(res, AppErrors.InvalidPath);
    }

    const task = parseTask(req.body);
    if (!task.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid body JSON");
      return sendError(res, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn(
        { id: id.data, method: "PUT", path: req.path },
        "Task not found"
      );
      return sendError(res, AppErrors.TaskNotFound);
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

    return res.status(200).json(task.data);
  };
}
