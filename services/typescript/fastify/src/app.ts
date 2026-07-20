import Fastify, { type FastifyInstance } from "fastify";
import { registerRoutes } from "./routes/router.js";
import type { State } from "app";
import { errorHandler, invalidPathHandler } from "./routes/handlers/index.js";
import { chaosHook, logHook } from "./routes/hooks/index.js";

const BYTES = 1024;

export function createApp(state: State): FastifyInstance {
  const app = Fastify({
    bodyLimit: state.config.request_size_limit * BYTES
  });
  app.addHook("onRequest", logHook);
  app.addHook("preHandler", chaosHook(state));
  registerRoutes(app, state);
  app.setNotFoundHandler(invalidPathHandler);
  app.setErrorHandler(errorHandler);

  return app;
}
