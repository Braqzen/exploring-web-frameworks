import { createRequire } from "node:module";
import type pino from "pino";

const require = createRequire(import.meta.url);

export function createLogger(serviceName: string): pino.Logger {
  const pino = require("pino") as typeof import("pino");

  return pino({
    name: serviceName,
    level: process.env.LOG_LEVEL ?? "info"
  });
}

export type Logger = ReturnType<typeof createLogger>;
