import type { RouteHandler } from "fastify";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "../../logger.js";

export const invalidPathHandler: RouteHandler = (request, reply) => {
  getLogger().warn(
    { method: request.method, path: request.url },
    "Invalid path"
  );
  sendError(reply, AppErrors.InvalidPath);
};
