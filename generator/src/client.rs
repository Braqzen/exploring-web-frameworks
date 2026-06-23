//! The client is used to send payload requests to the server.

use crate::payload::{Operation, Payload};
use eyre::Result;
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Histogram},
};
use opentelemetry_http::HeaderInjector;
use reqwest::{Client as ReqwestClient, header::HeaderMap};
use serde_json::json;
use std::time::{Duration, Instant};
use tracing::{Instrument, instrument};

pub struct Client {
    url: String,
    client: ReqwestClient,
    metrics: Metrics,
}

impl Client {
    pub fn new(url: String) -> Self {
        let metrics = Metrics::new();

        Self {
            url,
            client: ReqwestClient::new(),
            metrics,
        }
    }

    #[instrument(name = "client.post", err, skip(self))]
    pub async fn post(&self, payload: &Payload) -> Result<String> {
        self.metrics
            .record("POST", async {
                let response = self
                    .client
                    .post(&self.url)
                    .json(payload)
                    .headers(otel_headers())
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await?
                    .error_for_status()?;

                Ok(response
                    .json::<String>()
                    .instrument(tracing::info_span!("read_and_parse_json"))
                    .await?)
            })
            .await
    }

    #[instrument(name = "client.get", err, skip(self))]
    pub async fn get(&self, task_id: &str) -> Result<Payload> {
        let url = self.task_url(task_id);
        self.metrics
            .record("GET", async {
                let response = self
                    .client
                    .get(&url)
                    .headers(otel_headers())
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await?
                    .error_for_status()?;

                Ok(response
                    .json::<Payload>()
                    .instrument(tracing::info_span!("read_and_parse_json"))
                    .await?)
            })
            .await
    }

    #[instrument(name = "client.patch", err, skip(self))]
    pub async fn patch(&self, task_id: &str, operation: Operation) -> Result<Payload> {
        let url = self.task_url(task_id);
        self.metrics
            .record("PATCH", async {
                let response = self
                    .client
                    .patch(&url)
                    .json(&json!({ "operation": operation }))
                    .headers(otel_headers())
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await?
                    .error_for_status()?;

                Ok(response
                    .json::<Payload>()
                    .instrument(tracing::info_span!("read_and_parse_json"))
                    .await?)
            })
            .await
    }

    #[instrument(name = "client.put", err, skip(self))]
    pub async fn put(&self, task_id: &str, payload: Payload) -> Result<Payload> {
        let url = self.task_url(task_id);
        self.metrics
            .record("PUT", async {
                let response = self
                    .client
                    .put(&url)
                    .json(&payload)
                    .headers(otel_headers())
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await?
                    .error_for_status()?;

                Ok(response
                    .json::<Payload>()
                    .instrument(tracing::info_span!("read_and_parse_json"))
                    .await?)
            })
            .await
    }

    #[instrument(name = "client.delete", err, skip(self))]
    pub async fn delete(&self, task_id: &str) -> Result<()> {
        let url = self.task_url(task_id);
        self.metrics
            .record("DELETE", async {
                self.client
                    .delete(&url)
                    .headers(otel_headers())
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await?
                    .error_for_status()?;

                Ok(())
            })
            .await
    }

    fn task_url(&self, task_id: &str) -> String {
        format!("{}/{}", self.url.trim_end_matches('/'), task_id)
    }
}

struct Metrics {
    requests: Counter<u64>,
    duration: Histogram<f64>,
}

impl Metrics {
    fn new() -> Self {
        let meter = global::meter("client");
        let requests = meter.u64_counter("requests").build();
        let duration = meter.f64_histogram("duration").with_unit("s").build();

        Self { requests, duration }
    }

    fn increment_request(&self, method: &str) {
        self.requests
            .add(1, &[KeyValue::new("method", method.to_string())]);
    }

    fn record_duration(&self, method: &str, elapsed: Duration) {
        self.duration.record(
            elapsed.as_secs_f64(),
            &[KeyValue::new("method", method.to_string())],
        );
    }

    async fn record<T, F>(&self, method: &str, fut: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        // TODO: reqwest-middleware + reqwest-metrics instead
        self.increment_request(method);
        let start = Instant::now();
        let result = fut.await;
        self.record_duration(method, start.elapsed());
        result
    }
}

/// Injects the OTel context into the HTTP headers to associate the request with the trace.
fn otel_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    global::get_text_map_propagator(|prop| {
        prop.inject_context(
            &opentelemetry::Context::current(),
            &mut HeaderInjector(&mut headers),
        );
    });

    headers
}
