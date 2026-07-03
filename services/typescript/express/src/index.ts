import { createApp } from "./app.js";
import { startServer } from "./server.js";
import { type State } from "./state.js";

const port = process.env.PORT;

if (!port) {
  throw new Error("PORT is not set");
}

const state: State = {
  tasks: new Map()
};

startServer(createApp(state), parseInt(port));
