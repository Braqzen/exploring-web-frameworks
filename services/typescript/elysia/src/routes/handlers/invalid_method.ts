import type { Context } from "elysia";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export function invalidMethodHandler({ request, path, set }: Context) {
  getLogger().warn({ method: request.method, path }, "Invalid method");
  return sendError(set, AppErrors.InvalidMethod);
}
