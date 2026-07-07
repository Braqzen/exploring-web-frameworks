use crate::method::Method;
use eyre::Result;
use serde::Deserialize;

pub struct Config {
    /// Which Providers to send load to
    api: Vec<ProviderOptions>,
    /// How long to sleep between each request (miliseconds)
    sleep: u64,
}

impl Config {
    pub fn new() -> Result<Self> {
        let content = std::fs::read_to_string("/app/config.json")?;
        let config = serde_json::from_str::<ConfigJson>(&content)?;

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

            api.push(ProviderOptions {
                provider: provider.provider,
                language: provider.language,
                url: provider.url,
                methods,
            });
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
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Typescript,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::Rust => "rust".to_string(),
            Self::Typescript => "typescript".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigJson {
    /// Which Providers to send load to
    api: Vec<ApiJson>,
    /// How long to sleep between each request (miliseconds)
    sleep: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct ApiJson {
    /// Name to identify the provider
    provider: ProviderName,
    /// Programming language
    language: Language,
    /// URL to send requests to
    url: String,
    /// Whether the provider is loaded into the worker
    enabled: bool,
    /// Methods to send requests to
    methods: MethodsJson,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
struct MethodsJson {
    get: bool,
    post: bool,
    put: bool,
    delete: bool,
    patch: bool,
    head: bool,
}

impl MethodsJson {
    fn enabled(&self) -> Vec<Method> {
        [
            (self.get, Method::Get),
            (self.post, Method::Post),
            (self.put, Method::Put),
            (self.delete, Method::Delete),
            (self.patch, Method::Patch),
            (self.head, Method::Head),
        ]
        .into_iter()
        .filter(|(on, _)| *on)
        .map(|(_, method)| method)
        .collect()
    }
}
