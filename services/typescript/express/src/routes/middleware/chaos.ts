import type { RequestHandler, Request, Response, NextFunction } from "express";
import { randomInt } from "node:crypto";
import { setTimeout } from "node:timers/promises";
import { AppErrors, sendError } from "../errors.js";

export const chaosMiddleware: RequestHandler = async (
  _req: Request,
  res: Response,
  next: NextFunction
) => {
  if (randomInt(0, 101) < 5) {
    // Note: can't do sub ms but logic is same
    await setTimeout(randomInt(500, 1501) / 1000);
  }
  if (randomInt(0, 101) < 5) {
    return sendError(res, AppErrors.Internal);
  }

  return next();
};
