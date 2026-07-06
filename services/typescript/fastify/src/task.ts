import * as z from "zod";
import { Operation } from "./operation.js";

export const Task = z.object({
  secret: z.string(),
  operation: z.enum(Operation)
});

export type Task = z.infer<typeof Task>;

export const PatchedTask = z.object({
  operation: z.enum(Operation)
});

export type PatchedTask = z.infer<typeof PatchedTask>;
