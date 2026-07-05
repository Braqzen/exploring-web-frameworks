import type { RequestHandler, Request, Response, NextFunction } from "express";
import { logger } from "../../logger.js";

export const logMiddleware: RequestHandler = (
  req: Request,
  _res: Response,
  next: NextFunction
) => {
  let method = req.method;
  let path = req.url;

  logger.debug({ method, path }, "Incoming request");

  return next();
};
