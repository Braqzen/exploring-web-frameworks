//! The payload sent to a provider.
//!
//! Operation is a pointless variable used to put/patch requests and use in dashboards.

use crate::operation::Operation;
use serde::{Deserialize, Serialize};

/// A payload sent to the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    /// Arbitrary data to differentiate payloads.
    pub secret: String,

    /// The operation to perform on the payload.
    pub operation: Operation,
}

impl Payload {
    pub fn new(secret: String, operation: Operation) -> Self {
        Self { secret, operation }
    }
}
