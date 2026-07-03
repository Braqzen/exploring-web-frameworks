import type { Request, RequestHandler, Response } from "express";
import type { State } from "../../state.js";
import { z } from "zod";

export function getHandler(state: State): RequestHandler {
  return (req: Request, res: Response) => {
    const id = z.uuidv4().safeParse(req.params.id);
    if (!id.success) {
      return res.status(404).json({ error: "Invalid path" });
    }

    const task = state.tasks.get(id.data);
    if (!task) {
      return res.status(404).json({ error: "Task not found" });
    }

    return res.status(200).json(task);
  };
}
