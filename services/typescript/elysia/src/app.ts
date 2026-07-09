import { Elysia } from "elysia";
import { node } from "@elysia/node";
import type { State } from "app";
import { registerRoutes } from "./routes/router.js";
import { chaosHook, errorHook, logHook } from "./routes/hooks/index.js";

export function createApp(state: State): Elysia {
  const app = new Elysia({
    adapter: node(),
    serve: {
      maxRequestBodySize: 64 * 1024
    }
  })
    .use(logHook)
    .use(chaosHook)
    .use(errorHook);

  registerRoutes(app, state);

  return app;
}
