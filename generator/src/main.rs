mod api;
mod client;
mod methods;
mod operation;
mod payloads;
mod provider;
mod randomiser;
mod settings;
mod worker;

use crate::{settings::Config, worker::Worker};
use eyre::Result;
use telemetry::{Telemetry, cleanup};

#[tokio::main]
async fn main() -> Result<()> {
    let Telemetry {
        logger_provider,
        meter_provider,
        profiling_agent,
        tracer_provider,
    } = Telemetry::init("generator")?;

    let config = Config::new()?;

    let mut worker = Worker::new(config);

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
