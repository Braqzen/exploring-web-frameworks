import type { Express } from "express";
import type { Server } from "node:http";
import type { SocketAddress } from "node:net";
import { type Telemetry, getLogger } from "telemetry";

export function startServer(
  telemetry: Telemetry,
  app: Express,
  address: SocketAddress
): Server {
  const logger = getLogger();

  logger.info(
    { socket: `${address.address}:${address.port}` },
    "Starting router"
  );

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

  const server = app.listen(address.port, address.address, () => {
    logger.info(`Server is running on port ${address.port}`);
  });

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
