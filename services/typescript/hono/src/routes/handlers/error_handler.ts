import type { ErrorHandler } from "hono";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const errorHandler: ErrorHandler = (err, c) => {
  const logger = getLogger();

  if (
    err instanceof SyntaxError &&
    ["POST", "PUT", "PATCH"].includes(c.req.method)
  ) {
    logger.warn(
      { method: c.req.method, path: c.req.path },
      "Invalid body JSON"
    );

    return sendError(c, AppErrors.InvalidJsonBody);
  }

  logger.error(
    { method: c.req.method, path: c.req.path },
    "Internal server error"
  );

  return sendError(c, AppErrors.Internal);
};
