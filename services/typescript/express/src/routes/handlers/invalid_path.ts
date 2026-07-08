import type { RequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";
import { getLogger } from "telemetry";

export const invalidPathHandler: RequestHandler = (req, res) => {
  const logger = getLogger();
  logger.warn({ method: req.method, path: req.path }, "Invalid path");
  sendError(res, AppErrors.InvalidPath);
};
