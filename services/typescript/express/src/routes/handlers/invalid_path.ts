import type { RequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";

export const invalidPathHandler: RequestHandler = (_req, res) => {
  sendError(res, AppErrors.InvalidPath);
};
