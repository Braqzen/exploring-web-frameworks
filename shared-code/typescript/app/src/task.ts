import * as z from "zod";
import { TaskSchema, PatchedTaskSchema } from "./params.js";

export type Task = z.infer<typeof TaskSchema>;
export type PatchedTask = z.infer<typeof PatchedTaskSchema>;
