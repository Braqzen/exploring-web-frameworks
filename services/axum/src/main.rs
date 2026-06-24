mod api;
mod cli;
mod environment;
mod router;
mod server;
mod state;
mod task;

use cli::Cli;
use dotenvy::dotenv;
use environment::Environment;
use eyre::Result;
use rust_telemetry::{Telemetry, cleanup};
use server::Server;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init("axum")?;

    // Check for any CLI arguments to prioritize
    let args = match Cli::parse() {
        Ok(args) => args,
        Err(error) => {
            error!(%error, "Failed to parse CLI arguments");
            return Err(error);
        }
    };

    // Load environment variables from .env file
    dotenv().ok();

    // Parse environment variables and prioritize cli
    let environment = match Environment::new(&args) {
        Ok(environment) => environment,
        Err(error) => {
            error!(%error, "Failed to parse environment variables");
            return Err(error);
        }
    };

    let server = match Server::new(environment.socket).await {
        Ok(server) => server,
        Err(error) => {
            error!(%error, "Failed to create server");
            return Err(error);
        }
    };

    let profiling_agent = profiling_agent.start()?;

    let result = server.run().await;
    cleanup(
        &logger_provider,
        &meter_provider,
        profiling_agent,
        &tracer_provider,
    );

    result
}
