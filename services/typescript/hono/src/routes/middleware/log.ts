import type { Context, MiddlewareHandler, Next } from "hono";
import { getLogger } from "telemetry";

export const logMiddleware: MiddlewareHandler = (c: Context, next: Next) => {
  let method = c.req.method;
  let path = c.req.path;

  getLogger().debug({ method, path }, "Incoming request");

  return next();
};
