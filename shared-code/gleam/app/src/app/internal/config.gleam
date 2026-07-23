import gleam/dict.{type Dict}
import gleam/option.{type Option}

pub type ConfigJson {
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
