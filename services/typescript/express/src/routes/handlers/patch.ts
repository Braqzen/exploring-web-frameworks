import type { Request, RequestHandler, Response } from "express";
import { z } from "zod";
import type { State } from "../../state.js";
import { PatchedTask } from "../../task.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function patchHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid path");
      return sendError(res, AppErrors.InvalidPath);
    }

    const patchedTask = PatchedTask.safeParse(req.body);
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
