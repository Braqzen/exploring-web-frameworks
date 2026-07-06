import type { Request, RequestHandler, Response } from "express";
import { z } from "zod";
import type { State } from "../../state.js";
import { getLogger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function getHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid path");
      return sendError(res, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      logger.warn(
        { id: id.data, method: "GET", path: req.path },
        "Task not found"
      );
      return sendError(res, AppErrors.TaskNotFound);
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

    return res.status(200).json(task);
  };
}
