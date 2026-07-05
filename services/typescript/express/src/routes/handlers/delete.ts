import type { Request, RequestHandler, Response } from "express";
import { z } from "zod";
import type { State } from "../../state.js";
import { logger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function deleteHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      return sendError(res, AppErrors.InvalidPath);
    }

    const task = state.tasks.get(id.data);

    if (task === undefined) {
      logger.warn({ id: id.data, method: "DELETE" }, "Task not found");
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
