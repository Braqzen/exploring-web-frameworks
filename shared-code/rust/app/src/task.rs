use crate::operation::Operation;
use serde::{Deserialize, Serialize};

/// The payload expected by a web-framework from a POST/PUT request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    /// Arbitrary data to differentiate payloads.
    pub secret: String,

    /// The operation to perform on the payload.
    pub operation: Operation,
}

/// The payload expected by a web-framework from a PATCH request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PatchedTask {
    pub operation: Operation,
}
