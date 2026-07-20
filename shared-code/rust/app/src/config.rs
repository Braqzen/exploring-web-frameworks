use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ConfigJson {
    default: DefaultSettingsJson,
    #[serde(default)]
    overrides: HashMap<String, OverrideJson>,
}

#[derive(Debug, Deserialize, Clone)]
struct DefaultSettingsJson {
    latency: Settings,
    error: Settings,
    request_size_limit: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub enabled: bool,
    pub rate: u8,
}

#[derive(Debug, Deserialize, Clone, Default)]
struct OverrideJson {
    #[serde(default)]
    latency: Option<SettingsOverride>,
    #[serde(default)]
    error: Option<SettingsOverride>,
    #[serde(default)]
    request_size_limit: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, Default)]
struct SettingsOverride {
    #[serde(default)]
    enabled: Option<bool>,
    #[serde(default)]
    rate: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub latency: Settings,
    pub error: Settings,
    pub request_size_limit: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let service = std::env::var("SERVICE")?;
        let content = std::fs::read_to_string("/app/provider.json")?;
        let file = serde_json::from_str::<ConfigJson>(&content)?;

        let mut settings = file.default;

        if let Some(provider_overrides) = file.overrides.get(&service) {
            if let Some(latency) = &provider_overrides.latency {
                if let Some(enabled) = latency.enabled {
                    settings.latency.enabled = enabled;
                }
                if let Some(rate) = latency.rate {
                    settings.latency.rate = rate;
                }
            }

            if let Some(error) = &provider_overrides.error {
                if let Some(enabled) = error.enabled {
                    settings.error.enabled = enabled;
                }
                if let Some(rate) = error.rate {
                    settings.error.rate = rate;
                }
            }

            if let Some(limit) = provider_overrides.request_size_limit {
                settings.request_size_limit = limit;
            }
        }

        Ok(Self {
            latency: settings.latency,
            error: settings.error,
            request_size_limit: settings.request_size_limit,
        })
    }
}
