import { NodeSDK } from "@opentelemetry/sdk-node";
import { resourceFromAttributes } from "@opentelemetry/resources";
import { ATTR_SERVICE_NAME } from "@opentelemetry/semantic-conventions";
import { OTLPLogExporter } from "@opentelemetry/exporter-logs-otlp-http";
import { PinoInstrumentation } from "@opentelemetry/instrumentation-pino";
import { BatchLogRecordProcessor } from "@opentelemetry/sdk-logs";
import { createProfiler, type Profiler } from "./profiler.js";

export type Telemetry = {
  sdk: NodeSDK;
  profiler: Profiler;
};

export function initTelemetry(serviceName: string): Telemetry {
  const sdk = new NodeSDK({
    resource: resourceFromAttributes({
      [ATTR_SERVICE_NAME]: serviceName
    }),
    logRecordProcessors: [new BatchLogRecordProcessor(new OTLPLogExporter())],
    instrumentations: [new PinoInstrumentation()]
  });

  const profiler = createProfiler(serviceName);

  sdk.start();
  profiler.start();

  return {
    sdk,
    profiler
  };
}
