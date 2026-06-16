//! The primitive sent to the server for workers to process.

use rand::{
    RngExt,
    distr::{Alphanumeric, SampleString},
    rng,
};
use serde::Serialize;

/// A request sent to the server for workers to process.
#[derive(Debug, Serialize)]
pub struct Payload {
    /// Arbitrary data to differentiate requests.
    pub data: String,
}

impl Payload {
    pub fn new() -> Self {
        let data_size = rng().random_range(0..=32);
        let data = Alphanumeric
            .sample_string(&mut rng(), data_size)
            .to_string();

        Self { data }
    }
}
