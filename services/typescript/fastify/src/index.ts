import { SocketAddress } from "node:net";
import { createApp } from "./app.js";
import { startServer } from "./server.js";
import { createState, type State } from "app";
import { initTelemetry, initLogger } from "telemetry";

function main(): void {
  const addr = SocketAddress.parse(process.env.SOCKET ?? "");
  if (!addr) {
    throw new Error("SOCKET is not set or invalid");
  }

  const service = "fastify";
  // Create before init logger
  const telemetry = initTelemetry(service);
  initLogger(service);

  const state: State = createState();

  startServer(telemetry, createApp(state), addr);
}

main();
