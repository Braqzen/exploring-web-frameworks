import type { RequestHandler, Request, Response, NextFunction } from "express";
import { getLogger } from "telemetry";

export const logMiddleware: RequestHandler = (
  req: Request,
  _res: Response,
  next: NextFunction
) => {
  let method = req.method;
  let path = req.path;

  const logger = getLogger();

  logger.debug({ method, path }, "Incoming request");

  return next();
};
