use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Axum,
    Actix,
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Axum => "axum".to_string(),
            Provider::Actix => "actix".to_string(),
        }
    }
}
