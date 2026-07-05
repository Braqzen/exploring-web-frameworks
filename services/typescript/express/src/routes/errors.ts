import type { Response } from "express";

export const AppErrors = {
  TaskNotFound: { status: 404, message: "Task not found" },
  InvalidPath: { status: 404, message: "Invalid path" },
  InvalidMethod: { status: 405, message: "Invalid method" },
  InvalidJsonBody: { status: 422, message: "Invalid body JSON" },
  Internal: { status: 500, message: "Internal server error" }
} as const;

export type AppError = (typeof AppErrors)[keyof typeof AppErrors];

export function sendError(res: Response, error: AppError): Response {
  return res.status(error.status).json({ error: error.message });
}
