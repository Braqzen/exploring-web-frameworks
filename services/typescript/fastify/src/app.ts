import Fastify, { type FastifyInstance } from "fastify";
import { registerRoutes } from "./routes/router.js";
import type { State } from "app";
import { errorHandler, invalidPathHandler } from "./routes/handlers/index.js";
import { chaosHook, logHook } from "./routes/hooks/index.js";

export function createApp(state: State): FastifyInstance {
  const app = Fastify({ bodyLimit: 65536 });
  app.addHook("onRequest", logHook);
  app.addHook("preHandler", chaosHook);
  registerRoutes(app, state);
  app.setNotFoundHandler(invalidPathHandler);
  app.setErrorHandler(errorHandler);

  return app;
}
