use crate::{config::AppConfig, task::Task};
use std::collections::HashMap;
use uuid::Uuid;

pub struct AppState {
    pub tasks: HashMap<Uuid, Task>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            tasks: HashMap::new(),
            config,
        }
    }
}
