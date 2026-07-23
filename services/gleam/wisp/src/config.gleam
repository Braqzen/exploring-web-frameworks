import envoy
import gleam/dict.{type Dict}
import gleam/dynamic/decode
import gleam/json
import gleam/option.{type Option, None, Some}
import gleam/result
import simplifile

type ConfigJson {
  ConfigJson(
    default: DefaultSettingsJson,
    overrides: Dict(String, OverrideJson),
  )
}

pub type DefaultSettingsJson {
  DefaultSettingsJson(
    latency: Settings,
    error: Settings,
    request_size_limit: Int,
  )
}

pub type OverrideJson {
  OverrideJson(
    latency: Option(SettingsOverride),
    error: Option(SettingsOverride),
    request_size_limit: Option(Int),
  )
}

pub type Settings {
  Settings(enabled: Bool, rate: Int)
}

pub type SettingsOverride {
  SettingsOverride(enabled: Option(Bool), rate: Option(Int))
}

pub type AppConfig {
  AppConfig(latency: Settings, error: Settings, request_size_limit: Int)
}

pub type ConfigError {
  MissingService
  FileRead(simplifile.FileError)
  JsonDecode(json.DecodeError)
}

pub fn new_app_config() -> Result(AppConfig, ConfigError) {
  use service <- result.try(
    envoy.get("SERVICE") |> result.map_error(fn(_) { MissingService }),
  )
  use content <- result.try(
    simplifile.read(from: "/app/provider.json")
    |> result.map_error(FileRead),
  )
  use config <- result.try(
    json.parse(from: content, using: config_decoder())
    |> result.map_error(JsonDecode),
  )

  let default_settings = config.default

  let settings = case dict.get(config.overrides, service) {
    Error(_) -> default_settings
    Ok(override) -> {
      let latency = case override.latency {
        None -> default_settings.latency
        Some(override) ->
          Settings(
            enabled: option.unwrap(
              override.enabled,
              default_settings.latency.enabled,
            ),
            rate: option.unwrap(override.rate, default_settings.latency.rate),
          )
      }

      let error = case override.error {
        None -> default_settings.error
        Some(override) ->
          Settings(
            enabled: option.unwrap(
              override.enabled,
              default_settings.error.enabled,
            ),
            rate: option.unwrap(override.rate, default_settings.error.rate),
          )
      }

      let request_size_limit =
        option.unwrap(
          override.request_size_limit,
          default_settings.request_size_limit,
        )

      DefaultSettingsJson(latency, error, request_size_limit)
    }
  }

  Ok(AppConfig(
    latency: settings.latency,
    error: settings.error,
    request_size_limit: settings.request_size_limit,
  ))
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

fn default_settings_decoder() -> decode.Decoder(DefaultSettingsJson) {
  use latency <- decode.field("latency", settings_decoder())
  use error <- decode.field("error", settings_decoder())
  use request_size_limit <- decode.field("request_size_limit", decode.int)
  decode.success(DefaultSettingsJson(latency, error, request_size_limit))
}

fn config_decoder() -> decode.Decoder(ConfigJson) {
  use default <- decode.field("default", default_settings_decoder())
  use overrides <- decode.optional_field(
    "overrides",
    dict.new(),
    decode.dict(decode.string, override_decoder()),
  )
  decode.success(ConfigJson(default, overrides))
}
