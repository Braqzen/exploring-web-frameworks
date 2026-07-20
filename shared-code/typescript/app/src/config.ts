import { readFileSync } from "node:fs";
import * as z from "zod";

export type Config = {
  latency: Settings;
  error: Settings;
  request_size_limit: number;
};

export const Settings = z.object({
  enabled: z.boolean(),
  rate: z.number().int().min(0).max(255)
});
export type Settings = z.infer<typeof Settings>;

const DefaultSettingsJson = z.object({
  latency: Settings,
  error: Settings,
  request_size_limit: z.number().int().nonnegative()
});

const SettingsOverride = z.object({
  enabled: z.boolean().optional(),
  rate: z.number().int().min(0).max(255).optional()
});

const OverrideJson = z.object({
  latency: SettingsOverride.optional(),
  error: SettingsOverride.optional(),
  request_size_limit: z.number().int().nonnegative().optional()
});

const ConfigJson = z.object({
  default: DefaultSettingsJson,
  overrides: z.record(z.string(), OverrideJson).optional()
});

export function createConfig(): Config {
  const service = process.env["SERVICE"];
  if (service === undefined || service === "") {
    throw new Error("SERVICE env var is required");
  }

  let content: string;
  try {
    content = readFileSync("/app/provider.json", "utf8");
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e);
    throw new Error(`failed to read /app/provider.json: ${message}`);
  }

  let raw: unknown;
  try {
    raw = JSON.parse(content);
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e);
    throw new Error(`failed to parse provider.json: ${message}`);
  }

  const parsed = ConfigJson.safeParse(raw);
  if (!parsed.success) {
    throw new Error(`failed to parse provider.json: ${parsed.error.message}`);
  }

  const file = parsed.data;
  let latency = file.default.latency;
  let error = file.default.error;
  let request_size_limit = file.default.request_size_limit;

  const overrides = file.overrides ?? {};
  const providerOverrides = overrides[service];

  if (providerOverrides !== undefined) {
    if (providerOverrides.latency !== undefined) {
      let enabled = latency.enabled;
      let rate = latency.rate;

      if (providerOverrides.latency.enabled !== undefined) {
        enabled = providerOverrides.latency.enabled;
      }
      if (providerOverrides.latency.rate !== undefined) {
        rate = providerOverrides.latency.rate;
      }

      latency = { enabled, rate };
    }

    if (providerOverrides.error !== undefined) {
      let enabled = error.enabled;
      let rate = error.rate;

      if (providerOverrides.error.enabled !== undefined) {
        enabled = providerOverrides.error.enabled;
      }
      if (providerOverrides.error.rate !== undefined) {
        rate = providerOverrides.error.rate;
      }

      error = { enabled, rate };
    }

    if (providerOverrides.request_size_limit !== undefined) {
      request_size_limit = providerOverrides.request_size_limit;
    }
  }

  return {
    latency,
    error,
    request_size_limit
  };
}
