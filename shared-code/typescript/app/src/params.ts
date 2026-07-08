import * as z from "zod";
import { Operation } from "./operation.js";

export const TaskSchema = z.object({
  secret: z.string(),
  operation: z.enum(Operation)
});

export const PatchedTaskSchema = z.object({
  operation: z.enum(Operation)
});

export function parseId(value: unknown): z.ZodSafeParseResult<string> {
  return z.uuidv4().safeParse(value);
}

export function parseTask(
  value: unknown
): z.ZodSafeParseResult<z.infer<typeof TaskSchema>> {
  return TaskSchema.safeParse(value);
}

export function parsePatchedTask(
  value: unknown
): z.ZodSafeParseResult<z.infer<typeof PatchedTaskSchema>> {
  return PatchedTaskSchema.safeParse(value);
}
