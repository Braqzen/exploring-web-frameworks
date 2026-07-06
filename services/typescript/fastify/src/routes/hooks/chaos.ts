import type { onRequestAsyncHookHandler } from "fastify";
import { randomInt } from "node:crypto";
import { setTimeout } from "node:timers/promises";
import { AppErrors, sendError } from "../errors.js";

export const chaosHook: onRequestAsyncHookHandler = async (_request, reply) => {
  if (randomInt(0, 101) < 5) {
    // Note: can't do sub ms but logic is same
    await setTimeout(randomInt(500, 1501) / 1000);
  }
  if (randomInt(0, 101) < 5) {
    return sendError(reply, AppErrors.Internal);
  }
};
