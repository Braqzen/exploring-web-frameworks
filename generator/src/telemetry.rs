use eyre::Result;
use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider};
use pyroscope::{
    PyroscopeAgent,
    backend::{BackendConfig, PprofConfig, pprof_backend},
    pyroscope::{PyroscopeAgentBuilder, PyroscopeAgentReady, PyroscopeAgentRunning},
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Telemetry {
    pub logger_provider: SdkLoggerProvider,
    pub meter_provider: SdkMeterProvider,
    pub profiling_agent: PyroscopeAgent<PyroscopeAgentReady>,
}

impl Telemetry {
    pub fn init() -> Result<Self> {
        // Set the service name to be able to filter in dashboards by service.
        let resource = Resource::builder().with_service_name("generator").build();

        let logger_provider = Self::setup_logging(&resource)?;
        let meter_provider = Self::setup_metrics(&resource)?;
        let profiling_agent = Self::setup_profiling()?;

        Ok(Self {
            logger_provider,
            meter_provider,
            profiling_agent,
        })
    }

    fn setup_logging(resource: &Resource) -> Result<SdkLoggerProvider> {
        // When the logs are processed we need to export logs to the OTLP endpoint.
        let log_exporter = LogExporter::builder().with_http().build()?;

        // Handles batching and additional wiring to process logs before sending to exporter.
        let logging_provider = SdkLoggerProvider::builder()
            .with_batch_exporter(log_exporter)
            .with_resource(resource.clone())
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

        Ok(logging_provider)
    }

    fn setup_metrics(resource: &Resource) -> Result<SdkMeterProvider> {
        // When the metrics are processed we need to export metrics to the OTLP endpoint.
        let metric_exporter = MetricExporter::builder().with_http().build()?;

        // Handles aggregation and periodic push of metrics to the exporter.
        let metric_provider = SdkMeterProvider::builder()
            .with_periodic_exporter(metric_exporter)
            .with_resource(resource.clone())
            .build();

        // Similar to tracing's global subscriber, register the meter provider so modules can call global::meter().
        global::set_meter_provider(metric_provider.clone());

        Ok(metric_provider)
    }

    fn setup_profiling() -> Result<PyroscopeAgent<PyroscopeAgentReady>> {
        // Create an agent that will profile the whole process / application.
        let agent = PyroscopeAgentBuilder::new(
            std::env::var("PYROSCOPE_URL")?,
            "generator",
            100,
            "pyroscope-rs",
            env!("CARGO_PKG_VERSION"),
            pprof_backend(PprofConfig::default(), BackendConfig::default()),
        )
        .build()?;
        Ok(agent)
    }
}

pub fn cleanup(
    logger_provider: &SdkLoggerProvider,
    meter_provider: &SdkMeterProvider,
    profiling_agent: PyroscopeAgent<PyroscopeAgentRunning>,
) {
    if let Err(error) = logger_provider.shutdown() {
        eprintln!("logger provider otel shutdown failed: {error}");
    }
    if let Err(error) = meter_provider.shutdown() {
        eprintln!("metric provider otel shutdown failed: {error}");
    }
    match profiling_agent.stop() {
        Ok(agent) => agent.shutdown(),
        Err(error) => eprintln!("profiling agent stop failed: {error}"),
    }
}
