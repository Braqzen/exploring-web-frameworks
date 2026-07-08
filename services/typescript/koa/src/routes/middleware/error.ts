import type { Middleware } from "koa";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export const errorMiddleware: Middleware = async (ctx, next) => {
  try {
    await next();
  } catch (err) {
    const logger = getLogger();

    if (
      err instanceof Error &&
      ["POST", "PUT", "PATCH"].includes(ctx.req.method ?? "") &&
      (("status" in err && err.status === 413) || err instanceof SyntaxError)
    ) {
      logger.warn(
        { method: ctx.req.method, path: ctx.path },
        "Invalid body JSON"
      );

      sendError(ctx, AppErrors.InvalidJsonBody);
      return;
    }

    logger.error(
      { err, method: ctx.req.method, path: ctx.path },
      "Internal server error"
    );

    sendError(ctx, AppErrors.Internal);
  }
};
