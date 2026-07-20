import { SocketAddress } from "node:net";
import { createApp } from "./app.js";
import { startServer } from "./server.js";
import { createState, createConfig, type State } from "app";
import { initTelemetry, initLogger } from "telemetry";

function main(): void {
  const addr = SocketAddress.parse(process.env.SOCKET ?? "");
  if (!addr) {
    throw new Error("SOCKET is not set or invalid");
  }

  const service = process.env.SERVICE;
  if (service === undefined || service === "") {
    throw new Error("SERVICE is not set");
  }

  // Create before init logger
  const telemetry = initTelemetry(service);
  initLogger(service);

  const config = createConfig();
  const state: State = createState(config);

  startServer(telemetry, createApp(state), addr);
}

main();
