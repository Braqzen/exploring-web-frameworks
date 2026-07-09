import { Elysia } from "elysia";
import type { State } from "app";
import {
  postHandler,
  getHandler,
  putHandler,
  patchHandler,
  deleteHandler,
  invalidMethodHandler
} from "./handlers/index.js";

export function registerRoutes(app: Elysia, state: State): void {
  app
    .post("/", postHandler(state))
    .all("/", invalidMethodHandler)
    .get("/:id", getHandler(state))
    .put("/:id", putHandler(state))
    .patch("/:id", patchHandler(state))
    .delete("/:id", deleteHandler(state))
    .all("/:id", invalidMethodHandler);
}
