use crate::api::Provider;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ApiService {
    /// Name to identify the provider
    provider: Provider,
    /// URL to send requests to
    url: String,
    /// Whether the provider is loaded into the worker
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Which Providers to send load to
    api: Vec<ApiService>,
    /// How long to sleep between each request (miliseconds)
    sleep: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        let content = std::fs::read_to_string("/app/config.json")?;
        let mut config = serde_json::from_str::<Self>(&content)?;

        // Filter out disabled providers
        config.api = config
            .api
            .into_iter()
            .filter(|api| api.enabled)
            .collect::<Vec<_>>();

        if config.api.is_empty() {
            tracing::error!("No enabled APIs found in config.json");
            return Err(eyre::eyre!("No enabled APIs found in config.json"));
        }

        Ok(config)
    }

    pub fn api(&self) -> HashMap<Provider, String> {
        self.api
            .iter()
            .map(|api| (api.provider.clone(), api.url.clone()))
            .collect()
    }

    pub fn sleep(&self) -> u64 {
        self.sleep
    }
}
