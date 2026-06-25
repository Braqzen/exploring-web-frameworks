mod client;
mod payload;
mod worker;

use crate::{client::Client, worker::Worker};
use eyre::Result;
use rust_telemetry::{Telemetry, cleanup};

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init("generator")?;

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    let mut worker = Worker::new(client);

    let profiling_agent = profiling_agent.start()?;

    let result = worker.run().await;
    cleanup(
        &logger_provider,
        &meter_provider,
        profiling_agent,
        &tracer_provider,
    );

    result
}
