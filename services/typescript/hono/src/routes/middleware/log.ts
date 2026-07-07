import type { Context, MiddlewareHandler, Next } from "hono";
import { getLogger } from "../../logger.js";

export const logMiddleware: MiddlewareHandler = (c: Context, next: Next) => {
  let method = c.req.method;
  let path = c.req.path;

  const logger = getLogger();

  logger.debug({ method, path }, "Incoming request");

  return next();
};
