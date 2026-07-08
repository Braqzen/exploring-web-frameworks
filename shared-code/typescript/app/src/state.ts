import { type Task } from "./task.js";

export type State = {
  tasks: Map<string, Task>;
};

export function createState(): State {
  return {
    tasks: new Map()
  };
}
