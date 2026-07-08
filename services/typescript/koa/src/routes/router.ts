import Koa from "koa";
import Router from "@koa/router";
import type { State } from "app";
import {
  postHandler,
  getHandler,
  putHandler,
  patchHandler,
  deleteHandler
} from "./handlers/index.js";
import { invalidMethodMiddleware } from "./middleware/invalid_method.js";

export function registerRoutes(app: Koa, state: State): void {
  const router = new Router();

  router.post("/", postHandler(state));
  router.all("/", invalidMethodMiddleware);

  router.get("/:id", getHandler(state));
  router.put("/:id", putHandler(state));
  router.patch("/:id", patchHandler(state));
  router.delete("/:id", deleteHandler(state));
  router.all("/:id", invalidMethodMiddleware);

  app.use(router.routes());
  app.use(router.allowedMethods());
}
