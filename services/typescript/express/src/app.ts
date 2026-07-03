import express from "express";
import type { Express } from "express";
import type { State } from "./state.js";
import { registerRoutes } from "./routes/router.js";
import { validateRequest } from "./routes/middleware/validate_request.js";

export function createApp(state: State): Express {
  const app = express();

  app.use(express.json({ limit: "64kb" }));
  app.use(validateRequest);
  registerRoutes(app, state);

  return app;
}
