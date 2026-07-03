import type { Request, RequestHandler, Response } from "express";
import type { State } from "../../state.js";
import { PatchedTask } from "../../task.js";
import { z } from "zod";

export function patchHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      return res.status(404).json({ error: "Invalid path" });
    }

    const patchedTask = PatchedTask.safeParse(req.body);
    if (!patchedTask.success) {
      return res.status(400).json({ error: "Invalid body JSON" });
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      return res.status(404).json({ error: "Task not found" });
    }

    task.operation = patchedTask.data.operation;

    return res.status(200).json(task);
  };
}
