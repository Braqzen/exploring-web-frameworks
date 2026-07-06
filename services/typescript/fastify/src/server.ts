import type { FastifyInstance } from "fastify";
import type { Server } from "node:http";
import type { Telemetry } from "typescript-telemetry";
import { getLogger } from "./logger.js";

export function startServer(
  telemetry: Telemetry,
  app: FastifyInstance,
  port: number
): Server {
  const logger = getLogger();

  logger.info({ socket: `0.0.0.0:${port}` }, "Starting router");

  const shutdown = async (signal: string) => {
    const message =
      signal === "SIGINT"
        ? "Received interrupt signal"
        : signal === "STARTUP"
          ? "Shutting down after failed start"
          : "Received terminate signal";

    logger.info(message);

    await app.close();

    try {
      await telemetry.profiler.shutdown();
    } catch (error) {
      logger.error({ error }, "Profiler shutdown failed");
    }

    try {
      await telemetry.sdk.shutdown();
    } catch (error) {
      logger.error({ error }, "OpenTelemetry shutdown failed");
    }

    process.exit(0);
  };

  app.listen({ port, host: "0.0.0.0" }, (err, address) => {
    if (err) {
      logger.error({ err }, "Failed to start server");
      void shutdown("STARTUP");
      return;
    }
    logger.info(`Server is running on ${address}`);
  });

  process.once("SIGTERM", () => {
    void shutdown("SIGTERM");
  });

  process.once("SIGINT", () => {
    void shutdown("SIGINT");
  });

  return app.server;
}
