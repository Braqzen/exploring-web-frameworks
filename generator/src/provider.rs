use crate::settings::{Language, ProviderName};

pub struct Provider {
    name: ProviderName,
    url: String,
    language: Language,
}

impl Provider {
    pub fn new(name: ProviderName, url: String, language: Language) -> Self {
        Self {
            name,
            url,
            language,
        }
    }

    pub fn name(&self) -> ProviderName {
        self.name.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn language(&self) -> Language {
        self.language.clone()
    }
}
