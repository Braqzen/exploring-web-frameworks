import { NodeSDK } from "@opentelemetry/sdk-node";
import { resourceFromAttributes } from "@opentelemetry/resources";
import { ATTR_SERVICE_NAME } from "@opentelemetry/semantic-conventions";
import { OTLPLogExporter } from "@opentelemetry/exporter-logs-otlp-http";
import { PinoInstrumentation } from "@opentelemetry/instrumentation-pino";
import { BatchLogRecordProcessor } from "@opentelemetry/sdk-logs";

export function initTelemetry(serviceName: string): NodeSDK {
  const sdk = new NodeSDK({
    resource: resourceFromAttributes({
      [ATTR_SERVICE_NAME]: serviceName
    }),
    logRecordProcessors: [new BatchLogRecordProcessor(new OTLPLogExporter())],
    instrumentations: [new PinoInstrumentation()]
  });
  sdk.start();
  return sdk;
}

export type { NodeSDK } from "@opentelemetry/sdk-node";
