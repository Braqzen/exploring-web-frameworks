import type { RequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const invalidMethodHandler: RequestHandler = (req, res) => {
  const logger = getLogger();
  logger.warn({ method: req.method, path: req.path }, "Invalid method");
  sendError(res, AppErrors.InvalidMethod);
};
