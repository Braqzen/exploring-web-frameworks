use serde::Deserialize;

/// A payload sent to the server.
#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    /// Arbitrary data to differentiate payloads.
    pub secret: String,

    /// The operation to perform on the payload.
    pub operation: Operation,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Operation {
    Compute,
    Merge,
    Sort,
    Transform,
}

impl TryFrom<&str> for Operation {
    type Error = eyre::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "compute" => Ok(Operation::Compute),
            "merge" => Ok(Operation::Merge),
            "sort" => Ok(Operation::Sort),
            "transform" => Ok(Operation::Transform),
            _ => Err(eyre::eyre!("Invalid operation: {}", value)),
        }
    }
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
