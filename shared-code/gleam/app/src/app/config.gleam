import app/internal/config.{type Settings, DefaultSettingsJson, Settings}
import app/internal/config_decoders.{config_decoder}
import envoy
import gleam/dict
import gleam/json
import gleam/option.{None, Some}
import gleam/result
import simplifile

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
