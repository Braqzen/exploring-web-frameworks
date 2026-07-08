import Koa from "koa";
import { bodyParser } from "@koa/bodyparser";
import type { State } from "app";
import { registerRoutes } from "./routes/router.js";
import {
  chaosMiddleware,
  errorMiddleware,
  invalidPathMiddleware,
  logMiddleware
} from "./routes/middleware/index.js";

export function createApp(state: State): Koa {
  const app = new Koa();

  app.use(errorMiddleware);
  app.use(bodyParser({ jsonLimit: "64kb" }));
  app.use(logMiddleware);
  app.use(chaosMiddleware);
  registerRoutes(app, state);
  app.use(invalidPathMiddleware);

  return app;
}
