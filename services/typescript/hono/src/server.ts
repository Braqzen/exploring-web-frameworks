import type { Hono } from "hono";
import { serve, type ServerType } from "@hono/node-server";
import type { Telemetry } from "typescript-telemetry";
import { getLogger } from "./logger.js";

export function startServer(
  telemetry: Telemetry,
  app: Hono,
  port: number
): ServerType {
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

    await server[Symbol.asyncDispose]();

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

  const server = serve(
    { fetch: app.fetch, port, hostname: "0.0.0.0" },
    (info) => {
      logger.info(`Server is running on port ${info.port}`);
    }
  );

  server.on("error", (err) => {
    logger.error({ err }, "Failed to start server");
    void shutdown("STARTUP");
  });

  process.once("SIGTERM", () => {
    void shutdown("SIGTERM");
  });

  process.once("SIGINT", () => {
    void shutdown("SIGINT");
  });

  return server;
}
