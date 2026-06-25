//! The client is used to send payload requests to the server.

use crate::payload::{Operation, Payload};
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

    #[instrument(name = "client.post", err, skip_all)]
    pub async fn post(&self, payload: &Payload) -> Result<String> {
        let response = self
            .metrics
            .record("POST", async {
                self.client
                    .post(&self.url)
                    .json(payload)
                    .send()
                    .instrument(tracing::info_span!("send", method = "POST"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(response
            .json::<String>()
            .instrument(tracing::info_span!("read_and_parse_json"))
            .await?)
    }

    #[instrument(name = "client.get", err, skip_all)]
    pub async fn get(&self, task_id: &str) -> Result<Payload> {
        let url = self.task_url(task_id);
        let response = self
            .metrics
            .record("GET", async {
                self.client
                    .get(&url)
                    .send()
                    .instrument(tracing::info_span!("send", method = "GET"))
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
    pub async fn patch(&self, task_id: &str, operation: Operation) -> Result<Payload> {
        let url = self.task_url(task_id);
        let response = self
            .metrics
            .record("PATCH", async {
                self.client
                    .patch(&url)
                    .json(&json!({ "operation": operation }))
                    .send()
                    .instrument(tracing::info_span!("send", method = "PATCH"))
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
    pub async fn put(&self, task_id: &str, payload: Payload) -> Result<Payload> {
        let url = self.task_url(task_id);
        let response = self
            .metrics
            .record("PUT", async {
                self.client
                    .put(&url)
                    .json(&payload)
                    .send()
                    .instrument(tracing::info_span!("send", method = "PUT"))
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
    pub async fn delete(&self, task_id: &str) -> Result<()> {
        let url = self.task_url(task_id);
        self.metrics
            .record("DELETE", async {
                self.client
                    .delete(&url)
                    .send()
                    .instrument(tracing::info_span!("send", method = "DELETE"))
                    .await
            })
            .await?
            .error_for_status()?;

        Ok(())
    }

    fn task_url(&self, task_id: &str) -> String {
        format!("{}/{}", self.url.trim_end_matches('/'), task_id)
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
        // 1-3ms incrementing in 0.05ms, 3.1-4.9ms in 0.1ms, 5-15ms in 1ms
        let mut boundaries: Vec<f64> = (20..=60).map(|i| i as f64 * 0.05).collect();
        boundaries.extend((31..=49).map(|i| i as f64 * 0.1));
        boundaries.extend((5..=15).map(f64::from));

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

    fn increment_request(&self, method: &str) {
        self.requests
            .add(1, &[KeyValue::new("method", method.to_string())]);
    }

    fn record_duration(&self, method: &str, elapsed: Duration) {
        let ms = elapsed.as_secs_f64() * MILLISECONDS;
        let attrs = &[KeyValue::new("method", method.to_string())];

        self.latency.record(ms, attrs);
        self.latency_percentile.record(ms, attrs);
    }

    async fn record<T, E, F>(&self, method: &str, fut: F) -> std::result::Result<T, E>
    where
        F: Future<Output = std::result::Result<T, E>>,
    {
        self.increment_request(method);
        let start = Instant::now();
        let result = fut.await;
        self.record_duration(method, start.elapsed());
        result
    }
}
