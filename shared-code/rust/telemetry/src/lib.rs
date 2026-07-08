use eyre::Result;
use opentelemetry::global;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter};
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
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
    pub tracer_provider: SdkTracerProvider,
}

impl Telemetry {
    pub fn init(service_name: &str) -> Result<Self> {
        // Set the service name to be able to filter in dashboards by service.
        let resource = Resource::builder()
            .with_service_name(service_name.to_string())
            .build();

        // Tracer must be run before logging because we fetch the tracer and set it on the tracing_subscriber.
        let tracer_provider = Self::setup_tracing(&resource)?;
        let logger_provider = Self::setup_logging(&resource, service_name)?;
        let meter_provider = Self::setup_metrics(&resource)?;
        let profiling_agent = Self::setup_profiling(service_name)?;

        Ok(Self {
            logger_provider,
            meter_provider,
            profiling_agent,
            tracer_provider,
        })
    }

    fn setup_logging(resource: &Resource, service_name: &str) -> Result<SdkLoggerProvider> {
        // When the logs are processed we need to export logs to the OTLP endpoint.
        let log_exporter = LogExporter::builder().with_http().build()?;

        // Handles batching and additional wiring to process logs before sending to exporter.
        let logging_provider = SdkLoggerProvider::builder()
            .with_batch_exporter(log_exporter)
            .with_resource(resource.clone())
            .build();

        // Bridge "tracing" crate to OTeL SDK i.e. capture logs and send to provider.
        let convert_logs_to_otel = OpenTelemetryTracingBridge::new(&logging_provider);

        // Optional layer to continue to send logs to standard output.
        // Note the console format is different than the exported format.
        let send_logs_to_console = tracing_subscriber::fmt::layer().json().flatten_event(true);

        // Convert our spans to OTel spans.
        let convert_spans_to_otel =
            tracing_opentelemetry::layer().with_tracer(global::tracer(service_name.to_string()));

        // Init the tracing subscriber to export logs to ENDPOINT but also log to console for debugging.
        tracing_subscriber::registry()
            .with(convert_spans_to_otel)
            .with(convert_logs_to_otel)
            .with(send_logs_to_console)
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

    fn setup_profiling(service_name: &str) -> Result<PyroscopeAgent<PyroscopeAgentReady>> {
        // Create an agent that will profile the whole process / application.
        let agent = PyroscopeAgentBuilder::new(
            std::env::var("PYROSCOPE_URL")?,
            service_name.to_string(),
            100,
            "pyroscope-rs",
            env!("CARGO_PKG_VERSION"),
            pprof_backend(PprofConfig::default(), BackendConfig::default()),
        )
        .tags([("lang", "rust")].into_iter().collect())
        .build()?;
        Ok(agent)
    }

    fn setup_tracing(resource: &Resource) -> Result<SdkTracerProvider> {
        // When the traces are processed we need to export traces to the OTLP endpoint.
        let span_exporter = SpanExporter::builder().with_http().build()?;

        // Handles aggregation and periodic push of traces to the exporter.
        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .with_resource(resource.clone())
            .build();

        // Similar to tracing's global subscriber, register the tracer provider so modules can call global::tracer().
        global::set_tracer_provider(tracer_provider.clone());

        Ok(tracer_provider)
    }
}

pub fn cleanup(
    logger_provider: &SdkLoggerProvider,
    meter_provider: &SdkMeterProvider,
    profiling_agent: PyroscopeAgent<PyroscopeAgentRunning>,
    tracer_provider: &SdkTracerProvider,
) {
    if let Err(error) = logger_provider.shutdown() {
        eprintln!("logger provider otel shutdown failed: {error}");
    }
    if let Err(error) = meter_provider.shutdown() {
        eprintln!("metric provider otel shutdown failed: {error}");
    }
    if let Err(error) = tracer_provider.shutdown() {
        eprintln!("tracer provider otel shutdown failed: {error}");
    }
    match profiling_agent.stop() {
        Ok(agent) => agent.shutdown(),
        Err(error) => eprintln!("profiling agent stop failed: {error}"),
    }
}
