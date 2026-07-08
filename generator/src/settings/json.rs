//! Private types for how the config handles file parsing

use crate::{
    methods::Method,
    settings::{Language, ProviderName},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigJson {
    /// Which Providers to send load to
    pub api: Vec<ApiJson>,
    /// How long to sleep between each request (miliseconds)
    pub sleep: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiJson {
    /// Name to identify the provider
    pub provider: ProviderName,
    /// Programming language
    pub language: Language,
    /// URL to send requests to
    pub url: String,
    /// Whether the provider is loaded into the worker
    pub enabled: bool,
    /// Methods to send requests to
    pub methods: MethodsJson,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct MethodsJson {
    get: bool,
    post: bool,
    put: bool,
    delete: bool,
    patch: bool,
    head: bool,
}

impl MethodsJson {
    pub fn enabled(&self) -> Vec<Method> {
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
