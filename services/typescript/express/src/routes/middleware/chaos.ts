import type { RequestHandler, Request, Response, NextFunction } from "express";
import { randomInt } from "node:crypto";
import { setTimeout } from "node:timers/promises";
import type { State } from "app";
import { AppErrors, sendError } from "../errors.js";

export function chaosMiddleware(state: State): RequestHandler {
  return async (_req: Request, res: Response, next: NextFunction) => {
    const config = state.config;

    if (config.latency.enabled && randomInt(0, 101) < config.latency.rate) {
      // Note: can't do sub ms but logic is same
      await setTimeout(randomInt(500, 1501) / 1000);
    }
    if (config.error.enabled && randomInt(0, 101) < config.error.rate) {
      return sendError(res, AppErrors.Internal);
    }

    return next();
  };
}
