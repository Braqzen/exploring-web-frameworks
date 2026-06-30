//! The client is used to send payload requests to the server.

use crate::{
    api::Provider,
    payload::{Operation, Payload},
};
use eyre::Result;
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, Gauge, Histogram},
};
use reqwest::Client as ReqwestClient;
use serde_json::json;
use std::time::{Duration, Instant};
use tracing::{Instrument, instrument};

const MILLISECONDS: f64 = 1000.0;

pub struct Client {
    client: ReqwestClient,
    metrics: Metrics,
}

impl Client {
    pub fn new() -> Self {
        let metrics = Metrics::new();

        Self {
            client: ReqwestClient::new(),
            metrics,
        }
    }

    #[instrument(name = "client.post", err, skip_all)]
    pub async fn post(&self, provider: &Provider, url: &str, payload: &Payload) -> Result<String> {
        let response = self
            .metrics
            .record(provider, &payload.operation, "POST", async {
                self.client
                    .post(url)
                    .json(payload)
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(response
            .json::<CreatedTask>()
            .instrument(tracing::info_span!("read_and_parse_json"))
            .await?
            .id)
    }

    #[instrument(name = "client.get", err, skip_all)]
    pub async fn get(
        &self,
        provider: &Provider,
        url: &str,
        task_id: &str,
        operation: &Operation,
    ) -> Result<Payload> {
        let url = self.task_url(url, task_id);
        let response = self
            .metrics
            .record(provider, &operation, "GET", async {
                self.client
                    .get(&url)
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(response
            .json::<Payload>()
            .instrument(tracing::info_span!("read_and_parse_json"))
            .await?)
    }

    #[instrument(name = "client.patch", err, skip_all)]
    pub async fn patch(
        &self,
        provider: &Provider,
        url: &str,
        task_id: &str,
        operation: Operation,
    ) -> Result<Payload> {
        let url = self.task_url(url, task_id);
        let response = self
            .metrics
            .record(provider, &operation, "PATCH", async {
                self.client
                    .patch(&url)
                    .json(&json!({ "operation": operation }))
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(response
            .json::<Payload>()
            .instrument(tracing::info_span!("read_and_parse_json"))
            .await?)
    }

    #[instrument(name = "client.put", err, skip_all)]
    pub async fn put(
        &self,
        provider: &Provider,
        url: &str,
        task_id: &str,
        payload: Payload,
    ) -> Result<Payload> {
        let url = self.task_url(url, task_id);
        let response = self
            .metrics
            .record(provider, &payload.operation, "PUT", async {
                self.client
                    .put(&url)
                    .json(&payload)
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(response
            .json::<Payload>()
            .instrument(tracing::info_span!("read_and_parse_json"))
            .await?)
    }

    #[instrument(name = "client.delete", err, skip_all)]
    pub async fn delete(
        &self,
        provider: &Provider,
        url: &str,
        task_id: &str,
        operation: &Operation,
    ) -> Result<()> {
        let url = self.task_url(url, task_id);
        self.metrics
            .record(provider, &operation, "DELETE", async {
                self.client
                    .delete(&url)
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(())
    }

    #[instrument(name = "client.head", err, skip_all)]
    pub async fn head(
        &self,
        provider: &Provider,
        url: &str,
        task_id: &str,
        operation: &Operation,
    ) -> Result<()> {
        // Head is not implemented in frameworks and is intended to fail
        let url = self.task_url(url, task_id);
        self.metrics
            .record(provider, &operation, "HEAD", async {
                self.client
                    .head(&url)
                    .send()
                    .instrument(tracing::info_span!("send"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(())
    }

    fn task_url(&self, url: &str, task_id: &str) -> String {
        format!("{}/{}", url.trim_end_matches('/'), task_id)
    }
}

struct Metrics {
    // Tracks the number of requests sent to a server
    requests: Counter<u64>,
    // Tracks the latency 1 point at a time overwriting the previous point
    latency: Gauge<f64>,
    // Track the latency via buckets to calculate p50, p90, p99, etc.
    latency_percentile: Histogram<f64>,
}

impl Metrics {
    fn new() -> Self {
        let meter = global::meter("client");
        let requests = meter.u64_counter("requests").build();
        let latency = meter.f64_gauge("latency").with_unit("ms").build();

        // Create histogram bucket boundaries for metrics to be rounded into
        // This means we are not getting exact values for latency
        let boundaries: Vec<f64> = (1..=50).map(|i| i as f64 * 0.1).collect();

        let latency_percentile = meter
            .f64_histogram("latency_percentile")
            .with_boundaries(boundaries)
            .with_unit("ms")
            .build();

        Self {
            requests,
            latency,
            latency_percentile,
        }
    }

    fn increment_request(&self, provider: &Provider, operation: &Operation, method: &str) {
        self.requests.add(
            1,
            &[
                KeyValue::new("provider", provider.to_string()),
                KeyValue::new("operation", operation.to_string()),
                KeyValue::new("method", method.to_string()),
            ],
        );
    }

    fn record_duration(
        &self,
        provider: &Provider,
        operation: &Operation,
        method: &str,
        elapsed: Duration,
    ) {
        let ms = elapsed.as_secs_f64() * MILLISECONDS;
        let attrs = &[
            KeyValue::new("provider", provider.to_string()),
            KeyValue::new("operation", operation.to_string()),
            KeyValue::new("method", method.to_string()),
        ];

        self.latency.record(ms, attrs);
        self.latency_percentile.record(ms, attrs);
    }

    async fn record<T, E, F>(
        &self,
        provider: &Provider,
        operation: &Operation,
        method: &str,
        fut: F,
    ) -> std::result::Result<T, E>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        self.increment_request(provider, operation, method);
        let start = Instant::now();
        let result = fut.await;
        self.record_duration(provider, operation, method, start.elapsed());
        result
    }
}

#[derive(serde::Deserialize)]
struct CreatedTask {
    id: String,
}
