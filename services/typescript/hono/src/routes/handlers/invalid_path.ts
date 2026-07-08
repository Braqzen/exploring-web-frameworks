import type { NotFoundHandler } from "hono";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const invalidPathHandler: NotFoundHandler = (c) => {
  getLogger().warn({ method: c.req.method, path: c.req.path }, "Invalid path");
  return sendError(c, AppErrors.InvalidPath);
};
