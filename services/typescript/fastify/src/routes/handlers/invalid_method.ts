import type { RouteHandler } from "fastify";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "../../logger.js";

export const invalidMethodHandler: RouteHandler = (request, reply) => {
  const logger = getLogger();
  logger.warn({ method: request.method, path: request.url }, "Invalid method");
  sendError(reply, AppErrors.InvalidMethod);
};
