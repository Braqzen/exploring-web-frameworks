mod client;
mod payload;

use crate::{client::Client, payload::Payload};
use eyre::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber so we can observe the logs in case something goes wrong.
    // At this stage we do not care about logging as there will be no dashboards to view them.
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    // We need the generator to run indefinitely without too much spam in this stage so we'll send 1
    // type of payload at a fixed rate.
    loop {
        let payload = Payload::new();

        match client.post(&payload).await {
            Ok(response) => info!(data = payload.data, response, "Sent payload"),
            Err(error) => warn!(%error, "Failed to send payload"),
        }

        sleep(Duration::from_millis(10)).await;
    }
}
