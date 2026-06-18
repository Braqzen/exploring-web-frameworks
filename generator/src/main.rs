mod client;
mod payload;
mod worker;

use crate::{client::Client, worker::Worker};
use eyre::Result;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::LogExporter;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Set the service name to be able to filter in dashboards by service.
    let resource = Resource::builder().with_service_name("generator").build();

    // When the logs are processed we need to export logs to the OTLP endpoint.
    let log_exporter = LogExporter::builder().with_http().build()?;

    // Handles batching and additional wiring to process logs before sending to exporter.
    let logging_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(resource)
        .build();

    // Bridge "tracing" crate to OTeL SDK i.e. capture logs and send to provider.
    let otel_layer = OpenTelemetryTracingBridge::new(&logging_provider);

    // Init the tracing subscriber to export logs to ENDPOINT but also log to console for debugging.
    // Note the console format is different than the exported format.
    tracing_subscriber::registry()
        .with(otel_layer)
        .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
        .with(EnvFilter::from_default_env())
        .init();

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    let mut worker = Worker::new(client);

    tokio::select! {
        res = worker.run() => {
            if let Err(error) = logging_provider.shutdown() {
                eprintln!("logging provider otel shutdown failed: {error}");
            }

            res?;
        }
        _ = tokio::signal::ctrl_c() => {
            if let Err(error) = logging_provider.shutdown() {
                eprintln!("logging provider otel shutdown failed: {error}");
            }
        }
    }

    Ok(())
}
