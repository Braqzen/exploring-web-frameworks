import type { FastifyInstance } from "fastify";
import type { State } from "../state.js";
import {
  getHandler,
  postHandler,
  patchHandler,
  deleteHandler,
  putHandler,
  invalidMethodHandler
} from "./handlers/index.js";

export function registerRoutes(app: FastifyInstance, state: State) {
  app.post("/", postHandler(state));
  app.route({
    method: ["GET", "HEAD", "PUT", "PATCH", "DELETE", "OPTIONS", "TRACE"],
    url: "/",
    handler: invalidMethodHandler
  });

  app.get("/:id", getHandler(state));
  app.patch("/:id", patchHandler(state));
  app.delete("/:id", deleteHandler(state));
  app.put("/:id", putHandler(state));
  app.route({
    method: ["POST", "OPTIONS", "TRACE"],
    url: "/:id",
    handler: invalidMethodHandler
  });
}
