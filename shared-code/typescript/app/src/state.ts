import { type Config } from "./config.js";
import { type Task } from "./task.js";

export type State = {
  tasks: Map<string, Task>;
  config: Config;
};

export function createState(config: Config): State {
  return {
    tasks: new Map(),
    config
  };
}
