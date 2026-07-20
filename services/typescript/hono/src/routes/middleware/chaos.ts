import type { Context, MiddlewareHandler, Next } from "hono";
import { randomInt } from "node:crypto";
import { setTimeout } from "node:timers/promises";
import type { State } from "app";
import { AppErrors, sendError } from "../errors.js";

export function chaosMiddleware(state: State): MiddlewareHandler {
  return async (c: Context, next: Next) => {
    const config = state.config;

    if (config.latency.enabled && randomInt(0, 101) < config.latency.rate) {
      // Note: can't do sub ms but logic is same
      await setTimeout(randomInt(500, 1501) / 1000);
    }
    if (config.error.enabled && randomInt(0, 101) < config.error.rate) {
      return sendError(c, AppErrors.Internal);
    }

    return await next();
  };
}
