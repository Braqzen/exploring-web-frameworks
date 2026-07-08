import { type Request, type RequestHandler, type Response } from "express";
import { randomUUID } from "node:crypto";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";
import { parseTask, type State } from "app";

export function postHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const logger = getLogger();

    let task = parseTask(req.body);
    if (!task.success) {
      logger.warn({ method: req.method, path: req.path }, "Invalid body JSON");
      return sendError(res, AppErrors.InvalidJsonBody);
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

    return res.status(201).json({
      id: id
    });
  };
}
