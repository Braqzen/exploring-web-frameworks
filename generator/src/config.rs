use crate::api::Provider;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ApiService {
    provider: Provider,
    url: String,
    enabled: bool,
}

pub struct Config {
    apis: Vec<ApiService>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let content = std::fs::read_to_string("/app/config.json")?;
        let apis = serde_json::from_str::<Vec<ApiService>>(&content)?;
        let apis = apis
            .into_iter()
            .filter(|api| api.enabled)
            .collect::<Vec<_>>();

        if apis.is_empty() {
            tracing::error!("No enabled APIs found in config.json");
            return Err(eyre::eyre!("No enabled APIs found in config.json"));
        }

        Ok(Self { apis })
    }

    pub fn apis(&self) -> HashMap<Provider, String> {
        self.apis
            .iter()
            .map(|api| (api.provider.clone(), api.url.clone()))
            .collect()
    }
}
