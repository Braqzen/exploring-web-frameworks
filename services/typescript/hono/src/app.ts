import { Hono } from "hono";
import { bodyLimit } from "hono/body-limit";
import type { State } from "app";
import { registerRoutes } from "./routes/router.js";
import { invalidPathHandler, errorHandler } from "./routes/handlers/index.js";
import { chaosMiddleware, logMiddleware } from "./routes/middleware/index.js";
import { getLogger } from "telemetry";
import { AppErrors, sendError } from "./routes/errors.js";

const BYTES = 1024;

export function createApp(state: State): Hono {
  const app = new Hono();

  app.use(
    "*",
    bodyLimit({
      maxSize: state.config.request_size_limit * BYTES,
      onError: (c) => {
        getLogger().warn(
          { method: c.req.method, path: c.req.path },
          "Invalid body JSON"
        );
        return sendError(c, AppErrors.InvalidJsonBody);
      }
    })
  );
  app.use(logMiddleware);
  app.use(chaosMiddleware(state));
  registerRoutes(app, state);
  app.notFound(invalidPathHandler);
  app.onError(errorHandler);

  return app;
}
