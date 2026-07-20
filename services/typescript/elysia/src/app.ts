import { Elysia } from "elysia";
import { node } from "@elysia/node";
import type { State } from "app";
import { registerRoutes } from "./routes/router.js";
import { chaosHook, errorHook, logHook } from "./routes/hooks/index.js";

const BYTES = 1024;

export function createApp(state: State): Elysia {
  const app = new Elysia({
    adapter: node(),
    serve: {
      maxRequestBodySize: state.config.request_size_limit * BYTES
    }
  })
    .use(logHook)
    .use(chaosHook(state))
    .use(errorHook);

  registerRoutes(app, state);

  return app;
}
