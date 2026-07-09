import { Elysia } from "elysia";
import { getLogger } from "telemetry";

export const logHook = new Elysia({ name: "log" }).onBeforeHandle(
  ({ request, path }) => {
    getLogger().debug({ method: request.method, path }, "Incoming request");
  }
);
