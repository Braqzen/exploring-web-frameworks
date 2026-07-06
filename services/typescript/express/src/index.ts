import { createApp } from "./app.js";
import { startServer } from "./server.js";
import { type State } from "./state.js";
import { initTelemetry } from "typescript-telemetry";
import { initLogger } from "./logger.js";

function main(): void {
  const port = process.env.PORT;

  if (!port) {
    throw new Error("PORT is not set");
  }

  const service = "express";
  // Create before init logger
  const telemetry = initTelemetry(service);
  initLogger(service);

  const state: State = {
    tasks: new Map()
  };

  startServer(telemetry, createApp(state), parseInt(port));
}

main();
