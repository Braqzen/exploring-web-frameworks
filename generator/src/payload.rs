//! The payload sent to the server.
//!
//! Operation is a pointless variable used to put/patch requests and use in dashboards.

use rand::{
    RngExt,
    distr::{Alphanumeric, SampleString},
    rng,
};
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
    pub fn new() -> Self {
        let secret_size = rng().random_range(10..=32);
        let secret = Alphanumeric
            .sample_string(&mut rng(), secret_size)
            .to_string();

        let operation = match rng().random_range(0..=3) {
            0 => Operation::Compute,
            1 => Operation::Merge,
            2 => Operation::Sort,
            3 => Operation::Transform,
            _ => unreachable!(),
        };

        Self { secret, operation }
    }
}

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
