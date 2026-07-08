use crate::task::Task;
use std::collections::HashMap;
use uuid::Uuid;

pub struct AppState {
    pub tasks: HashMap<Uuid, Task>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}
