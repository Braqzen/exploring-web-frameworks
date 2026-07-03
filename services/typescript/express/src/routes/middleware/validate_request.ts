import type { RequestHandler, Request, Response, NextFunction } from "express";

export const validateRequest: RequestHandler = (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  let method = req.method;

  if (
    method !== "POST" &&
    method !== "PUT" &&
    method !== "PATCH" &&
    method !== "DELETE" &&
    method !== "GET"
  ) {
    return res.status(405).json({ error: "Method not allowed" });
  }

  return next();
};
