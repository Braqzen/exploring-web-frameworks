import type { Express } from "express";
import type { State } from "../state.js";
import {
  postHandler,
  getHandler,
  putHandler,
  patchHandler,
  deleteHandler
} from "./handlers/index.js";

export function registerRoutes(app: Express, state: State) {
  app.post("/", postHandler(state));
  app
    .route("/:id")
    .get(getHandler(state))
    .put(putHandler(state))
    .patch(patchHandler(state))
    .delete(deleteHandler(state));
}
