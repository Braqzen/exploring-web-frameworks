import type { RequestHandler } from "express";
import { AppErrors, sendError } from "../errors.js";

export const invalidMethodHandler: RequestHandler = (_req, res) => {
  sendError(res, AppErrors.InvalidMethod);
};
