use crate::{
    api::{ApiManager, Method, Provider},
    client::Client,
    config::Config,
    payload::{Operation, Payload},
};
use eyre::Result;
use opentelemetry::{KeyValue, global, metrics::UpDownCounter};
use rand::{RngExt, rng};
use std::time::Duration;
use tokio::{
    signal::unix::{SignalKind, signal},
    time::sleep,
};
use tracing::{field::Empty, info, instrument, warn};

pub struct Worker {
    api_manager: ApiManager,
    client: Client,
    metrics: Metrics,
    sleep: u64,
}

impl Worker {
    pub fn new(config: Config) -> Self {
        Self {
            api_manager: ApiManager::new(config.api()),
            client: Client::new(),
            metrics: Metrics::new(),
            sleep: config.sleep(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Handle running locally and interrupting the process with ctrl+c.
        let mut sigint = signal(SignalKind::interrupt())?;
        // Handle running in a container and terminating the process with docker stop.
        let mut sigterm = signal(SignalKind::terminate())?;

        let shutdown = async {
            tokio::select! {
                _ = sigint.recv() => info!("Received interrupt signal"),
                _ = sigterm.recv() => info!("Received terminate signal"),
            }
        };
        tokio::pin!(shutdown);

        loop {
            // Ignore error try again
            let _ = self.send_request().await;

            tokio::select! {
                _ = &mut shutdown => return Ok(()),
                _ = sleep(Duration::from_millis(self.sleep)) => {},
            }
        }
    }

    #[instrument(name = "worker.send_request", skip_all, fields(provider = Empty, method = Empty))]
    async fn send_request(&mut self) -> Result<()> {
        let (provider, url, method) = self.api_manager.select();
        tracing::Span::current().record("provider", provider.to_string());
        tracing::Span::current().record("method", method.to_string());

        match method {
            Method::Post => self.post(provider, url).await?,
            Method::Get => self.get(provider, url).await?,
            Method::Patch => self.patch(provider, url).await?,
            Method::Put => self.put(provider, url).await?,
            Method::Delete => self.delete(provider, url).await?,
        };
        Ok(())
    }

    #[instrument(name = "worker.post", err, skip_all)]
    async fn post(&mut self, provider: Provider, url: String) -> Result<()> {
        let payload = Payload::new();

        match self.client.post(&provider, &url, &payload).await {
            Ok(id) => {
                self.metrics
                    .increment_operation(&provider, &payload.operation.to_string());

                let _ = self.insert(&provider, &id, &payload)?;

                info!(
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    id,
                    method = "POST",
                    provider = provider.to_string(),
                    "Stored task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(
                    %error,
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    method = "POST",
                    provider = provider.to_string(),
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.get", err, skip_all)]
    async fn get(&mut self, provider: Provider, url: String) -> Result<()> {
        let (task_id, payload) = self.payload(&provider)?;

        match self
            .client
            .get(&provider, &url, &task_id, &payload.operation)
            .await
        {
            Ok(task) => {
                info!(
                    secret = task.secret,
                    operation = task.operation.to_string(),
                    id = task_id,
                    method = "GET",
                    provider = provider.to_string(),
                    "Retrieved task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(
                    %error,
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    id = task_id,
                    method = "GET",
                    provider = provider.to_string(),
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.patch", err, skip_all)]
    async fn patch(&mut self, provider: Provider, url: String) -> Result<()> {
        let (task_id, payload) = self.payload(&provider)?;

        // TODO: enforce a different operation than the current one
        let random_operation = rng().random_range(0..=3);
        let operation = match random_operation {
            0 => Operation::Compute,
            1 => Operation::Merge,
            2 => Operation::Sort,
            3 => Operation::Transform,
            _ => unreachable!(),
        };

        match self
            .client
            .patch(&provider, &url, &task_id, operation.clone())
            .await
        {
            Ok(task) => {
                self.metrics
                    .increment_operation(&provider, &operation.to_string());
                self.metrics
                    .decrement_operation(&provider, &payload.operation.to_string());

                self.api_manager.insert(&provider, &task_id, &task);

                info!(
                    secret = payload.secret,
                    from_operation = payload.operation.to_string(),
                    to_operation = task.operation.to_string(),
                    id = task_id,
                    method = "PATCH",
                    provider = provider.to_string(),
                    "Patched task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(
                    %error,
                    secret = payload.secret,
                    from_operation = payload.operation.to_string(),
                    to_operation = operation.to_string(),
                    id = task_id,
                    method = "PATCH",
                    provider = provider.to_string(),
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.put", err, skip_all)]
    async fn put(&mut self, provider: Provider, url: String) -> Result<()> {
        let (task_id, old_payload) = self.payload(&provider)?;

        let new_payload = Payload::new();

        match self
            .client
            .put(&provider, &url, &task_id, new_payload.clone())
            .await
        {
            Ok(task) => {
                self.metrics
                    .increment_operation(&provider, &new_payload.operation.to_string());
                self.metrics
                    .decrement_operation(&provider, &old_payload.operation.to_string());

                let _ = self.insert(&provider, &task_id, &task)?;

                info!(
                    from_secret = old_payload.secret,
                    to_secret = new_payload.secret,
                    from_operation = old_payload.operation.to_string(),
                    to_operation = new_payload.operation.to_string(),
                    id = task_id,
                    method = "PUT",
                    provider = provider.to_string(),
                    "Put task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(
                    %error,
                    from_secret = old_payload.secret,
                    to_secret = new_payload.secret,
                    from_operation = old_payload.operation.to_string(),
                    to_operation = new_payload.operation.to_string(),
                    id = task_id,
                    method = "PUT",
                    provider = provider.to_string(),
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.delete", err, skip_all)]
    async fn delete(&mut self, provider: Provider, url: String) -> Result<()> {
        let (task_id, payload) = self.payload(&provider)?;

        match self
            .client
            .delete(&provider, &url, &task_id, &payload.operation)
            .await
        {
            Ok(_) => {
                let _ = self.remove(&provider, &task_id)?;
                self.metrics
                    .decrement_operation(&provider, &payload.operation.to_string());

                info!(
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    id = task_id,
                    method = "DELETE",
                    provider = provider.to_string(),
                    "Deleted task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(%error, task_id, method = "DELETE", provider = provider.to_string(), "Failed client request");
                Err(error)
            }
        }
    }

    fn payload(&self, provider: &Provider) -> Result<(String, Payload)> {
        match self.api_manager.payload(&provider) {
            Some((task_id, payload)) => return Ok((task_id, payload)),
            None => {
                return {
                    warn!(
                        provider = provider.to_string(),
                        "Missing payload for provider",
                    );
                    Err(eyre::eyre!(
                        "Missing payload for provider {}",
                        provider.to_string()
                    ))
                };
            }
        };
    }

    fn remove(&mut self, provider: &Provider, task_id: &String) -> Result<Payload> {
        match self.api_manager.remove(&provider, &task_id) {
            Some(payload) => Ok(payload),
            None => {
                return {
                    warn!(
                        provider = provider.to_string(),
                        "Failed to remove payload for provider",
                    );
                    Err(eyre::eyre!(
                        "Failed to remove payload for provider {}",
                        provider.to_string()
                    ))
                };
            }
        }
    }

    fn insert(&mut self, provider: &Provider, id: &String, payload: &Payload) -> Result<Payload> {
        match self.api_manager.insert(&provider, &id, &payload) {
            Some(payload) => Ok(payload),
            None => {
                return {
                    warn!(
                        provider = provider.to_string(),
                        "Failed to insert payload for provider",
                    );
                    Err(eyre::eyre!(
                        "Failed to insert payload for provider {}",
                        provider.to_string()
                    ))
                };
            }
        }
    }
}

struct Metrics {
    /// Number of operations in memory per API and operation type
    operations: UpDownCounter<i64>,
}

impl Metrics {
    fn new() -> Self {
        let meter = global::meter("worker");

        let operations = meter.i64_up_down_counter("live_operations").build();

        Self { operations }
    }

    fn increment_operation(&self, provider: &Provider, operation: &str) {
        self.operations.add(
            1,
            &[
                KeyValue::new("provider", provider.to_string()),
                KeyValue::new("operation", operation.to_string()),
            ],
        );
    }

    fn decrement_operation(&self, provider: &Provider, operation: &str) {
        self.operations.add(
            -1,
            &[
                KeyValue::new("provider", provider.to_string()),
                KeyValue::new("operation", operation.to_string()),
            ],
        );
    }
}
