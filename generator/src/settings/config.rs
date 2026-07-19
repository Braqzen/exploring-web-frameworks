//! Public API for handling the config

use crate::{
    methods::Method,
    settings::json::{ConfigJson, MetaJson, ProviderJson},
};
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;

pub struct Config {
    /// Which Providers to send load to
    api: Vec<ProviderOptions>,
    /// How long to sleep between each request (miliseconds)
    sleep: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config = std::fs::read_to_string("/app/config.json")?;
        let config = serde_json::from_str::<ConfigJson>(&config)?;

        let meta = std::fs::read_to_string("/app/meta.json")?;
        let meta = serde_json::from_str::<Vec<MetaJson>>(&meta)?;

        let meta: HashMap<ProviderName, MetaJson> =
            meta.into_iter().map(|m| (m.provider.clone(), m)).collect();

        let mut api = Vec::new();
        for provider in config.api {
            if !provider.enabled {
                continue;
            }

            let methods = provider.methods.enabled();
            if methods.is_empty() {
                tracing::error!(
                    provider = provider.provider.to_string(),
                    "All HTTP methods disabled for provider"
                );
                continue;
            }

            let Some(meta) = meta.get(&provider.provider).cloned() else {
                tracing::error!(
                    provider = provider.provider.to_string(),
                    "No meta entry for provider"
                );
                continue;
            };

            api.push(ProviderOptions::new(provider, meta, methods));
        }

        if api.is_empty() {
            tracing::error!("No enabled APIs with enabled HTTP methods found in config.json");
            return Err(eyre::eyre!(
                "No enabled APIs with enabled HTTP methods found in config.json"
            ));
        }

        Ok(Self {
            api,
            sleep: config.sleep,
        })
    }

    pub fn api(&self) -> Vec<ProviderOptions> {
        self.api.clone()
    }

    pub fn sleep(&self) -> u64 {
        self.sleep
    }
}

#[derive(Debug, Clone)]
pub struct ProviderOptions {
    pub provider: ProviderName,
    pub language: Language,
    pub url: String,
    pub methods: Vec<Method>,
}

impl ProviderOptions {
    fn new(provider: ProviderJson, meta: MetaJson, methods: Vec<Method>) -> Self {
        Self {
            provider: provider.provider,
            language: meta.language,
            url: meta.url,
            methods,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProviderName {
    Axum,
    Actix,
    Warp,
    Rocket,
    Poem,
    Salvo,
    Express,
    Fastify,
    Hono,
    Koa,
    Elysia,
    Flask,
    FastApi,
    Sanic,
    Quart,
    Django,
    Tornado,
    Starlette,
    Gin,
    Chi,
    Fiber,
    Echo,
}

impl ToString for ProviderName {
    fn to_string(&self) -> String {
        match self {
            Self::Axum => "axum".to_string(),
            Self::Actix => "actix".to_string(),
            Self::Warp => "warp".to_string(),
            Self::Rocket => "rocket".to_string(),
            Self::Poem => "poem".to_string(),
            Self::Salvo => "salvo".to_string(),
            Self::Express => "express".to_string(),
            Self::Fastify => "fastify".to_string(),
            Self::Hono => "hono".to_string(),
            Self::Koa => "koa".to_string(),
            Self::Elysia => "elysia".to_string(),
            Self::Flask => "flask".to_string(),
            Self::FastApi => "fastapi".to_string(),
            Self::Sanic => "sanic".to_string(),
            Self::Quart => "quart".to_string(),
            Self::Django => "django".to_string(),
            Self::Tornado => "tornado".to_string(),
            Self::Starlette => "starlette".to_string(),
            Self::Gin => "gin".to_string(),
            Self::Chi => "chi".to_string(),
            Self::Fiber => "fiber".to_string(),
            Self::Echo => "echo".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Typescript,
    Python,
    Go,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::Rust => "rust".to_string(),
            Self::Typescript => "typescript".to_string(),
            Self::Python => "python".to_string(),
            Self::Go => "go".to_string(),
        }
    }
}
