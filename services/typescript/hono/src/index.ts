import { SocketAddress } from "node:net";
import { type State, createState } from "app";
import { initTelemetry, initLogger } from "telemetry";
import { createApp } from "./app.js";
import { startServer } from "./server.js";

function main(): void {
  const addr = SocketAddress.parse(process.env.SOCKET ?? "");
  if (!addr) {
    throw new Error("SOCKET is not set or invalid");
  }

  const service = "hono";
  // Create before init logger
  const telemetry = initTelemetry(service);
  initLogger(service);

  const state: State = createState();

  startServer(telemetry, createApp(state), addr);
}

main();
