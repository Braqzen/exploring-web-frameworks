import express from "express";
import type { Express } from "express";
import type { State } from "./state.js";
import { registerRoutes } from "./routes/router.js";
import { invalidPathHandler } from "./routes/handlers/index.js";
import {
  chaosMiddleware,
  logMiddleware,
  errorMiddleware
} from "./routes/middleware/index.js";

export function createApp(state: State): Express {
  const app = express();

  app.use(express.json({ limit: "64kb" }));
  app.use(logMiddleware);
  app.use(chaosMiddleware);
  registerRoutes(app, state);
  app.use(invalidPathHandler);
  app.use(errorMiddleware);

  return app;
}
