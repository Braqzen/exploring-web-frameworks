mod client;
mod payload;
mod telemetry;
mod worker;

use crate::{
    client::Client,
    telemetry::{Telemetry, cleanup},
    worker::Worker,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
    } = Telemetry::init()?;

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    let mut worker = Worker::new(client);

    let profiling_agent = profiling_agent.start()?;

    tokio::select! {
        res = worker.run() => {
            cleanup(&logger_provider, &meter_provider, profiling_agent);

            res?;
        }
        _ = tokio::signal::ctrl_c() => {
            cleanup(&logger_provider, &meter_provider, profiling_agent);
        }
    }

    Ok(())
}
