import { type Request, type RequestHandler, type Response } from "express";
import { type State } from "../../state.js";
import { Task } from "../../task.js";
import { randomUUID } from "node:crypto";

export function postHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    let task = Task.safeParse(req.body);
    if (!task.success) {
      return res.status(400).json({ error: "Invalid body JSON" });
    }

    let id = randomUUID();

    state.tasks.set(id, task.data);

    return res.status(201).json({
      id: id
    });
  };
}
