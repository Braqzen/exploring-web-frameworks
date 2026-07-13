import { Elysia } from "elysia";
import type { SocketAddress } from "node:net";
import { type Telemetry, getLogger } from "telemetry";

export function startServer(
  telemetry: Telemetry,
  app: Elysia,
  address: SocketAddress
): Elysia {
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

    await app.stop();

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

  app.listen({ port: address.port, hostname: address.address }, () => {
    logger.info(`Server is running on port ${address.port}`);
  });

  process.once("SIGTERM", () => {
    void shutdown("SIGTERM");
  });

  process.once("SIGINT", () => {
    void shutdown("SIGINT");
  });

  return app;
}
