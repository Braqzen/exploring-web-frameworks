use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Compute,
    Merge,
    Sort,
    Transform,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Compute => "compute".to_string(),
            Operation::Merge => "merge".to_string(),
            Operation::Sort => "sort".to_string(),
            Operation::Transform => "transform".to_string(),
        }
    }
}
