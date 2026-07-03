import type { Request, RequestHandler, Response } from "express";
import type { State } from "../../state.js";
import { Task } from "../../task.js";
import { z } from "zod";

export function putHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      return res.status(404).json({ error: "Invalid path" });
    }

    const task = Task.safeParse(req.body);
    if (!task.success) {
      return res.status(400).json({ error: "Invalid body JSON" });
    }

    if (!state.tasks.has(id.data)) {
      return res.status(404).json({ error: "Task not found" });
    }

    state.tasks.set(id.data, task.data);

    return res.status(200).json(task.data);
  };
}
