import type { Handler } from "hono";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const invalidMethodHandler: Handler = (c) => {
  getLogger().warn(
    { method: c.req.method, path: c.req.path },
    "Invalid method"
  );
  return sendError(c, AppErrors.InvalidMethod);
};
