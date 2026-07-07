import type { Handler } from "hono";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "../../logger.js";

export const invalidMethodHandler: Handler = (c) => {
  const logger = getLogger();
  logger.warn({ method: c.req.method, path: c.req.path }, "Invalid method");
  return sendError(c, AppErrors.InvalidMethod);
};
