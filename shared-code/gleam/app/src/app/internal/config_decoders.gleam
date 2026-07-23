import app/internal/config.{
  type ConfigJson, type DefaultSettingsJson, type OverrideJson, type Settings,
  type SettingsOverride, ConfigJson, DefaultSettingsJson, OverrideJson, Settings,
  SettingsOverride,
}
import gleam/dict
import gleam/dynamic/decode
import gleam/option.{None}

pub fn config_decoder() -> decode.Decoder(ConfigJson) {
  use default <- decode.field("default", default_settings_decoder())
  use overrides <- decode.optional_field(
    "overrides",
    dict.new(),
    decode.dict(decode.string, override_decoder()),
  )
  decode.success(ConfigJson(default, overrides))
}

fn default_settings_decoder() -> decode.Decoder(DefaultSettingsJson) {
  use latency <- decode.field("latency", settings_decoder())
  use error <- decode.field("error", settings_decoder())
  use request_size_limit <- decode.field("request_size_limit", decode.int)
  decode.success(DefaultSettingsJson(latency, error, request_size_limit))
}

fn settings_decoder() -> decode.Decoder(Settings) {
  use enabled <- decode.field("enabled", decode.bool)
  use rate <- decode.field("rate", decode.int)
  decode.success(Settings(enabled, rate))
}

fn settings_override_decoder() -> decode.Decoder(SettingsOverride) {
  use enabled <- decode.optional_field(
    "enabled",
    None,
    decode.optional(decode.bool),
  )
  use rate <- decode.optional_field("rate", None, decode.optional(decode.int))
  decode.success(SettingsOverride(enabled, rate))
}

fn override_decoder() -> decode.Decoder(OverrideJson) {
  use latency <- decode.optional_field(
    "latency",
    None,
    decode.optional(settings_override_decoder()),
  )

  use error <- decode.optional_field(
    "error",
    None,
    decode.optional(settings_override_decoder()),
  )

  use request_size_limit <- decode.optional_field(
    "request_size_limit",
    None,
    decode.optional(decode.int),
  )
  decode.success(OverrideJson(latency, error, request_size_limit))
}
