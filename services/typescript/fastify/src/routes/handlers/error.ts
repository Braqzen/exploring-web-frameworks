import type { FastifyError, FastifyReply, FastifyRequest } from "fastify";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "../../logger.js";

export function errorHandler(
  error: FastifyError,
  request: FastifyRequest,
  reply: FastifyReply
): FastifyReply | void {
  if (reply.sent) return;

  const logger = getLogger();

  if (
    error.code === "FST_ERR_CTP_BODY_TOO_LARGE" ||
    error.code === "FST_ERR_CTP_INVALID_JSON_BODY"
  ) {
    logger.warn(
      { method: request.method, path: request.url },
      "Invalid body JSON"
    );

    return sendError(reply, AppErrors.InvalidJsonBody);
  }

  logger.error(
    { method: request.method, path: request.url },
    "Internal server error"
  );

  return sendError(reply, AppErrors.Internal);
}
