import type { ErrorRequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const errorMiddleware: ErrorRequestHandler = (err, req, res, next) => {
  if (res.headersSent) {
    return next(err);
  }

  const logger = getLogger();

  if (err.type === "entity.too.large" || err.name === "PayloadTooLargeError") {
    logger.warn({ method: req.method, path: req.path }, "Invalid body JSON");

    return sendError(res, AppErrors.InvalidJsonBody);
  }

  if (err instanceof SyntaxError && "body" in err) {
    logger.warn({ method: req.method, path: req.path }, "Invalid body JSON");

    return sendError(res, AppErrors.InvalidJsonBody);
  }

  logger.error({ method: req.method, path: req.path }, "Internal server error");

  return sendError(res, AppErrors.Internal);
};
