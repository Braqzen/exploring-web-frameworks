import type { Context, Middleware, Next } from "koa";
import { getLogger } from "telemetry";

export const logMiddleware: Middleware = (ctx: Context, next: Next) => {
  let method = ctx.request.method;
  let path = ctx.request.path;

  getLogger().debug({ method, path }, "Incoming request");

  return next();
};
