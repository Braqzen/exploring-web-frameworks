//! Private types for how the config handles file parsing

use crate::{
    methods::Method,
    settings::{Language, ProviderName},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ConfigJson {
    /// Defaults applied to every provider in meta.json
    pub default: ProviderJson,
    #[serde(default)]
    pub overrides: HashMap<ProviderName, ProviderOverrideJson>,
    /// How long to sleep between each request (miliseconds)
    pub sleep: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProviderJson {
    /// Whether the provider is loaded into the worker
    pub enabled: bool,
    /// Methods to send requests to
    pub methods: MethodsJson,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ProviderOverrideJson {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub methods: MethodsOverrideJson,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(default, rename_all = "UPPERCASE")]
pub struct MethodsOverrideJson {
    pub get: Option<bool>,
    pub post: Option<bool>,
    pub put: Option<bool>,
    pub delete: Option<bool>,
    pub patch: Option<bool>,
    pub head: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaJson {
    /// Name to identify the provider
    pub provider: ProviderName,
    /// Programming language
    pub language: Language,
    /// URL to send requests to
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct MethodsJson {
    pub get: bool,
    pub post: bool,
    pub put: bool,
    pub delete: bool,
    pub patch: bool,
    pub head: bool,
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
