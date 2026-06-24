mod client;
mod payload;
mod worker;

use crate::{client::Client, worker::Worker};
use eyre::Result;
use rust_telemetry::{Telemetry, cleanup};
use tokio::signal::unix::{SignalKind, signal};
use tracing::error;

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

    // Handle running locally and interrupting the process with ctrl+c.
    let mut sigint = match signal(SignalKind::interrupt()) {
        Ok(sigint) => sigint,
        Err(error) => {
            error!(%error, "Failed to create interrupt signal");
            cleanup(
                &logger_provider,
                &meter_provider,
                profiling_agent,
                &tracer_provider,
            );
            return Err(eyre::eyre!(error));
        }
    };

    // Handle running in a container and terminating the process with docker stop.
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(sigterm) => sigterm,
        Err(error) => {
            error!(%error, "Failed to create terminate signal");
            cleanup(
                &logger_provider,
                &meter_provider,
                profiling_agent,
                &tracer_provider,
            );
            return Err(eyre::eyre!(error));
        }
    };

    tokio::select! {
        res = worker.run() => {
            cleanup(&logger_provider, &meter_provider, profiling_agent, &tracer_provider);

            res?;
        }
        _ = sigint.recv() => {
            cleanup(&logger_provider, &meter_provider, profiling_agent, &tracer_provider);
        }
        _ = sigterm.recv() => {
            cleanup(&logger_provider, &meter_provider, profiling_agent, &tracer_provider);
        }
    }

    Ok(())
}
