import type { ErrorRequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";

export const errorMiddleware: ErrorRequestHandler = (err, _req, res, next) => {
  if (res.headersSent) {
    return next(err);
  }

  if (err.type === "entity.too.large" || err.name === "PayloadTooLargeError") {
    return sendError(res, AppErrors.InvalidJsonBody);
  }

  if (err instanceof SyntaxError && "body" in err) {
    return sendError(res, AppErrors.InvalidJsonBody);
  }

  return sendError(res, AppErrors.Internal);
};
