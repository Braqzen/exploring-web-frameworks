import type { Middleware } from "koa";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const invalidPathMiddleware: Middleware = (ctx) => {
  getLogger().warn({ method: ctx.req.method, path: ctx.path }, "Invalid path");

  sendError(ctx, AppErrors.InvalidPath);
};
