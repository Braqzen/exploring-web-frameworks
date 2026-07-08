import type { Request, RequestHandler, Response } from "express";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";
import { parseId, parsePatchedTask, type State } from "app";

export function patchHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    const id = parseId(req.params.id);
    if (!id.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid path");
      return sendError(res, AppErrors.InvalidPath);
    }

    const patchedTask = parsePatchedTask(req.body);
    if (!patchedTask.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid body JSON");
      return sendError(res, AppErrors.InvalidJsonBody);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "PATCH", path: req.path },
        "Task not found"
      );
      return sendError(res, AppErrors.TaskNotFound);
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

    return res.status(200).json(task);
  };
}
