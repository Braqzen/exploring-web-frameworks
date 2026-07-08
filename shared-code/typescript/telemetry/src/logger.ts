import { createRequire } from "node:module";
import type pino from "pino";

const require = createRequire(import.meta.url);

let logger: pino.Logger | undefined;

export function initLogger(serviceName: string): void {
  const pino = require("pino") as typeof import("pino");

  logger = pino({
    name: serviceName,
    level: process.env.LOG_LEVEL ?? "info"
  });
}

export function getLogger(): pino.Logger {
  if (!logger) {
    throw new Error("Logger is not initialized");
  }

  return logger;
}
