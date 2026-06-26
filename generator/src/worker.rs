use crate::{
    api::Provider,
    client::Client,
    config::Config,
    payload::{Operation, Payload},
};
use eyre::Result;
use opentelemetry::{KeyValue, global, metrics::UpDownCounter};
use rand::{
    RngExt, rng,
    seq::{IndexedRandom, IteratorRandom},
};
use std::{collections::HashMap, time::Duration};
use tokio::{
    signal::unix::{SignalKind, signal},
    time::sleep,
};
use tracing::{info, instrument, warn};

pub struct Worker {
    client: Client,
    api_manager: ApiManager,
    metrics: Metrics,
}

impl Worker {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            api_manager: ApiManager::new(config.apis()),
            metrics: Metrics::new(),
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
            let (provider, url, method) = self.api_manager.select();

            match method {
                Method::Post => self.post(provider, url).await?,
                Method::Get => self.get(provider, url).await?,
                Method::Patch => self.patch(provider, url).await?,
                Method::Put => self.put(provider, url).await?,
                Method::Delete => self.delete(provider, url).await?,
            };

            tokio::select! {
                _ = &mut shutdown => return Ok(()),
                _ = sleep(Duration::from_millis(100)) => {},
            }
        }
    }

    #[instrument(name = "worker.post", err, skip_all)]
    async fn post(&mut self, provider: Provider, url: String) -> Result<()> {
        let payload = Payload::new();

        match self.client.post(&provider, &url, &payload).await {
            Ok(id) => {
                self.metrics
                    .increment_operation(&provider, &payload.operation.to_string());

                self.api_manager.insert(&provider, &id, &payload);

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
        let (task_id, payload) = self.api_manager.payload(&provider);

        match self.client.get(&provider, &url, &task_id).await {
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
        let (task_id, payload) = self.api_manager.payload(&provider);

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
        let (task_id, old_payload) = self.api_manager.payload(&provider);

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

                self.api_manager.insert(&provider, &task_id, &task);

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
        let (task_id, payload) = self.api_manager.payload(&provider);

        match self.client.delete(&provider, &url, &task_id).await {
            Ok(_) => {
                self.api_manager.remove(&provider, &task_id);
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
}

struct ApiManager {
    apis: HashMap<Provider, ApiState>,
}

// TODO: This currently assumes worker passes provider around so safe to unwrap but API will
//       panic if config excludes Provider and they pass that 1 in
impl ApiManager {
    fn new(apis: HashMap<Provider, String>) -> Self {
        let apis = apis
            .into_iter()
            .map(|(provider, url)| {
                (
                    provider,
                    ApiState {
                        url,
                        tasks: HashMap::new(),
                    },
                )
            })
            .collect();

        Self { apis }
    }

    fn select(&self) -> (Provider, String, Method) {
        // SAFETY: We hold exlusive access to apis and it's non-empty therefore cannot panic
        let (provider, state) = self.apis.iter().choose(&mut rng()).unwrap();

        if state.tasks.is_empty() {
            (provider.clone(), state.url.clone(), Method::Post)
        } else {
            // SAFETY: REQUESTS is non-empty and has values greater than 0
            let (method, _) = REQUESTS
                .choose_weighted(&mut rng(), |(_, weight)| *weight)
                .unwrap();

            (provider.clone(), state.url.clone(), method.to_owned())
        }
    }

    fn insert(&mut self, provider: &Provider, id: &String, payload: &Payload) {
        // SAFETY: Currently unsafe, can panic
        self.apis
            .get_mut(&provider)
            .unwrap()
            .tasks
            .insert(id.clone(), payload.clone());
    }

    fn remove(&mut self, provider: &Provider, task_id: &String) {
        // SAFETY: Currently unsafe, can panic
        self.apis
            .get_mut(&provider)
            .unwrap()
            .tasks
            .remove(task_id)
            .unwrap();
    }

    fn payload(&self, provider: &Provider) -> (String, Payload) {
        // SAFETY: Currently unsafe, can panic
        let (task_id, payload) = self
            .apis
            .get(provider)
            .unwrap()
            .tasks
            .iter()
            .choose(&mut rng())
            .unwrap();

        (task_id.clone(), payload.clone())
    }
}

struct ApiState {
    /// URL of the container to send a request to
    url: String,
    /// Task ID -> Payload
    tasks: HashMap<String, Payload>,
}

/// Distribution mapping the method to a weight percentage for random selection.
///
/// I.e. call this method X% of the time.
const REQUESTS: [(Method, u32); 5] = [
    (Method::Post, 40),   // 40%
    (Method::Get, 25),    // 25%
    (Method::Patch, 15),  // 15%
    (Method::Put, 10),    // 10%
    (Method::Delete, 10), // 10%
];

/// The method of the HTTP request to send to a server.
#[derive(Clone, PartialEq)]
enum Method {
    Post,
    Get,
    Patch,
    Put,
    Delete,
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
