use crate::config::ProviderName;

pub struct Provider {
    name: ProviderName,
    url: String,
}

impl Provider {
    pub fn new(name: ProviderName, url: String) -> Self {
        Self { name, url }
    }

    pub fn name(&self) -> ProviderName {
        self.name.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}
