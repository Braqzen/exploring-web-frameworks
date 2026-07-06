import type { PatchedTask, Task } from "../task.js";

export type IdParams = {
  Params: { id: string };
};

export type PostRoute = {
  Body: Task;
};

export type PutRoute = IdParams & {
  Body: Task;
};

export type PatchRoute = IdParams & {
  Body: PatchedTask;
};

export type GetRoute = IdParams;
export type DeleteRoute = IdParams;
