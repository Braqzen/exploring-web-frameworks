import type { Express } from "express";
import type { Server } from "node:http";
import { logger } from "./logger.js";

export function startServer(app: Express, port: number): Server {
  logger.info("Starting router");

  const server = app.listen(port, () => {
    logger.info(`Server is running on port ${port}`);
  });

  const shutdown = (signal: string) => {
    logger.info(`${signal} received, shutting down`);

    server.close((err) => {
      if (err) {
        logger.error(err);
        process.exit(1);
      }
      process.exit(0);
    });

    setTimeout(() => {
      logger.error("Forced shutdown after timeout");
      process.exit(1);
    }, 10_000).unref();
  };

  process.on("SIGTERM", () => shutdown("SIGTERM"));
  process.on("SIGINT", () => shutdown("SIGINT"));

  return server;
}
