use crate::task::Task;
use std::collections::HashMap;
use uuid::Uuid;

pub struct State {
    pub tasks: HashMap<Uuid, Task>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}
