mod client;
mod payload;
mod telemetry;
mod worker;

use crate::{client::Client, telemetry::Telemetry, worker::Worker};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
    } = Telemetry::init()?;

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    let mut worker = Worker::new(client);

    tokio::select! {
        res = worker.run() => {
            if let Err(error) = logger_provider.shutdown() {
                eprintln!("logger provider otel shutdown failed: {error}");
            }
            if let Err(error) = meter_provider.shutdown() {
                eprintln!("metric provider otel shutdown failed: {error}");
            }

            res?;
        }
        _ = tokio::signal::ctrl_c() => {
            if let Err(error) = logger_provider.shutdown() {
                eprintln!("logger provider otel shutdown failed: {error}");
            }
            if let Err(error) = meter_provider.shutdown() {
                eprintln!("metric provider otel shutdown failed: {error}");
            }
        }
    }

    Ok(())
}
