import { type State, createState } from "app";
import { initTelemetry, initLogger } from "telemetry";
import { createApp } from "./app.js";
import { startServer } from "./server.js";

function main(): void {
  const port = process.env.PORT;

  if (!port) {
    throw new Error("PORT is not set");
  }

  const service = "hono";
  // Create before init logger
  const telemetry = initTelemetry(service);
  initLogger(service);

  const state: State = createState();

  startServer(telemetry, createApp(state), parseInt(port));
}

main();
