//! The payload sent to a provider.
//!
//! Operation is a pointless variable used to put/patch requests and use in dashboards.

use crate::{operation::Operation, randomiser::Randomiser};
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
    seq::IndexedRandom,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::error;

pub struct PayloadManager {
    operations: Arc<Mutex<[(Operation, u16); 5]>>,
}

impl PayloadManager {
    pub fn new() -> Self {
        // Randomly set the operations for the first time
        let random_operations = Randomiser::operations();

        // Enable background mutation of requests
        let operations = Arc::new(Mutex::new(random_operations));

        // Sprinkle in the fun part of rust to make the compiler happy
        let task_operations = Arc::clone(&operations);

        // Spawn a background task (not a thread) which periodically randomises the request distribution
        tokio::spawn(async move {
            loop {
                Randomiser::sleep().await;

                {
                    let operations = Randomiser::operations();

                    match task_operations.lock() {
                        Ok(mut guard) => {
                            *guard = operations;
                        }
                        Err(error) => {
                            error!(%error, "Poisoned request lock in background randomiser task of MethodManager");

                            // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                            let mut guard = error.into_inner();
                            *guard = operations;
                            task_operations.clear_poison();
                        }
                    }
                }
            }
        });

        Self { operations }
    }

    pub fn payload(&self) -> Payload {
        let secret = self.secret();
        let operation = self.operation();

        Payload::new(secret, operation)
    }

    fn secret(&self) -> String {
        let secret_size = Randomiser::secret_size();

        Alphanumeric
            .sample_string(&mut rng(), secret_size)
            .to_string()
    }

    fn operation(&self) -> Operation {
        let guard = match self.operations.lock() {
            Ok(guard) => guard,
            Err(error) => {
                error!(%error, "Poisoned operation lock in select method of PayloadManager");

                // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                let operations = Randomiser::operations();

                let mut guard = error.into_inner();
                *guard = operations;
                self.operations.clear_poison();

                guard
            }
        };

        // SAFETY: OPERATIONS is non-empty and has values greater than 0
        let (operation, _) = guard
            .choose_weighted(&mut rng(), |(_, weight)| *weight)
            .unwrap();

        operation.clone()
    }
}

/// A payload sent to the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    /// Arbitrary data to differentiate payloads.
    pub secret: String,

    /// The operation to perform on the payload.
    pub operation: Operation,
}

impl Payload {
    fn new(secret: String, operation: Operation) -> Self {
        Self { secret, operation }
    }
}
