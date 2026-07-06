import Pyroscope from "@pyroscope/nodejs";

export type Profiler = {
  start: () => void;
  shutdown: () => Promise<void>;
};

export function createProfiler(serviceName: string): Profiler {
  Pyroscope.init({
    serverAddress: process.env.PYROSCOPE_URL,
    appName: serviceName,
    flushIntervalMs: 10000,
    wall: {
      samplingDurationMs: 10000,
      samplingIntervalMicros: 1000,
      collectCpuTime: true
    },
    tags: {
      lang: "node"
    }
  });

  return {
    start: () => Pyroscope.start(),
    shutdown: () => Pyroscope.stop()
  };
}
