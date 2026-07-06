import type { Express } from "express";
import type { Server } from "node:http";
import type { NodeSDK } from "typescript-telemetry";
import { getLogger } from "./logger.js";

export function startServer(sdk: NodeSDK, app: Express, port: number): Server {
  const logger = getLogger();

  logger.info({ socket: `0.0.0.0:${port}` }, "Starting router");

  const server = app.listen(port, () => {
    logger.info(`Server is running on port ${port}`);
  });

  const shutdown = async (signal: string) => {
    const message =
      signal === "SIGINT"
        ? "Received interrupt signal"
        : "Received terminate signal";

    logger.info(message);

    await server[Symbol.asyncDispose]();
    await sdk.shutdown();

    process.exit(0);
  };

  process.once("SIGTERM", () => {
    void shutdown("SIGTERM");
  });

  process.once("SIGINT", () => {
    void shutdown("SIGINT");
  });

  return server;
}
