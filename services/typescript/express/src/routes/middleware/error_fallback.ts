import type { ErrorRequestHandler } from "express";

export const errorMiddleware: ErrorRequestHandler = (err, _req, res, next) => {
  if (res.headersSent) {
    return next(err);
  }

  if (err.type === "entity.too.large" || err.name === "PayloadTooLargeError") {
    return res.status(422).json({ error: "Invalid body JSON" });
  }

  if (err instanceof SyntaxError && "body" in err) {
    return res.status(422).json({ error: "Invalid body JSON" });
  }

  return res.status(500).json({ error: "Internal server error" });
};
