import type { Request, RequestHandler, Response } from "express";
import { parseId, type State } from "app";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export function deleteHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    const id = parseId(req.params.id);
    if (!id.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid path");
      return sendError(res, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);

    if (task === undefined) {
      logger.warn(
        { id: id.data, method: "DELETE", path: req.path },
        "Task not found"
      );
      return sendError(res, AppErrors.TaskNotFound);
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

    return res.status(204).send();
  };
}
