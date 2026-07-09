import { Elysia } from "elysia";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "../errors.js";

export const errorHook = new Elysia({ name: "error" }).onError(
  ({ code, error, request, path, set }) => {
    const logger = getLogger();

    if (code === "NOT_FOUND") {
      logger.warn({ method: request.method, path }, "Invalid path");
      return sendError(set, AppErrors.InvalidPath);
    }

    if (
      (code === "PARSE" || error instanceof SyntaxError) &&
      ["POST", "PUT", "PATCH"].includes(request.method)
    ) {
      logger.warn({ method: request.method, path }, "Invalid body JSON");
      return sendError(set, AppErrors.InvalidJsonBody);
    }

    logger.error(
      { error, method: request.method, path },
      "Internal server error"
    );

    return sendError(set, AppErrors.Internal);
  }
);
