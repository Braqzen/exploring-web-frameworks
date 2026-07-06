import type { onRequestAsyncHookHandler } from "fastify";
import { getLogger } from "../../logger.js";

export const logHook: onRequestAsyncHookHandler = async (request, _reply) => {
  const logger = getLogger();

  logger.debug(
    { method: request.method, path: request.url },
    "Incoming request"
  );
};
