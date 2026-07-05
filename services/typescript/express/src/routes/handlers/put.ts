import type { Request, RequestHandler, Response } from "express";
import { z } from "zod";
import type { State } from "../../state.js";
import { Task } from "../../task.js";
import { logger } from "../../logger.js";
import { AppErrors, sendError } from "../errors.js";

export function putHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      return res.status(404).json({ error: "Invalid path" });
    }

    const task = Task.safeParse(req.body);
    if (!task.success) {
      return sendError(res, AppErrors.InvalidJsonBody);
    }

    const previous_task = state.tasks.get(id.data);

    if (previous_task === undefined) {
      logger.warn({ id: id.data, method: "PUT" }, "Task not found");
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
