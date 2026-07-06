import { createLogger, type Logger } from "typescript-telemetry";

let logger: Logger | undefined;

export function initLogger(serviceName: string): void {
  logger = createLogger(serviceName);
}

export function getLogger(): Logger {
  if (!logger) {
    throw new Error("Logger is not initialized");
  }

  return logger;
}
