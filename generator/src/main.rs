mod client;
mod payload;
mod worker;

use crate::{client::Client, worker::Worker};
use eyre::Result;
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

    let mut worker = Worker::new(client);
    worker.run().await?;

    Ok(())
}
