import type { Hono } from "hono";
import type { State } from "app";
import {
  postHandler,
  getHandler,
  putHandler,
  patchHandler,
  deleteHandler,
  invalidMethodHandler
} from "./handlers/index.js";

export function registerRoutes(app: Hono, state: State) {
  app.post("/", postHandler(state));
  app.all("/", invalidMethodHandler);

  app
    .get("/:id", getHandler(state))
    .put(putHandler(state))
    .patch(patchHandler(state))
    .delete(deleteHandler(state))
    .all(invalidMethodHandler);
}
