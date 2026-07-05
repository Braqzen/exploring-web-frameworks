import pino from "pino";

export function createLogger(serviceName: string): pino.Logger {
  return pino({
    name: serviceName,
    level: process.env.LOG_LEVEL ?? "info"
  });
}

export type Logger = ReturnType<typeof createLogger>;
