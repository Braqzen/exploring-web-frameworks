use crate::{
    client::Client,
    payload::{Operation, Payload},
};
use eyre::Result;
use opentelemetry::{
    KeyValue, global,
    metrics::{Counter, UpDownCounter},
};
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
    tasks: HashMap<String, Payload>,
    metrics: Metrics,
}

impl Worker {
    pub fn new(client: Client) -> Self {
        let metrics = Metrics::new();

        Self {
            client,
            tasks: HashMap::new(),
            metrics,
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
            let (method, _) = REQUESTS.choose_weighted(&mut rng(), |(_, weight)| *weight)?;

            match method {
                Method::Post => self.post().await?,
                Method::Get => self.get().await?,
                Method::Patch => self.patch().await?,
                Method::Put => self.put().await?,
                Method::Delete => self.delete().await?,
            };

            tokio::select! {
                _ = &mut shutdown => return Ok(()),
                _ = sleep(Duration::from_millis(100)) => {},
            }
        }
    }

    #[instrument(name = "worker.post", err, skip(self))]
    async fn post(&mut self) -> Result<()> {
        let payload = Payload::new();

        match self.client.post(&payload).await {
            Ok(id) => {
                self.metrics
                    .increment_total_operation(&payload.operation.to_string());
                self.metrics
                    .increment_live_operation(&payload.operation.to_string());

                self.tasks.insert(id.clone(), payload.clone());

                info!(
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    id,
                    method = "POST",
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
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.get", err, skip(self))]
    async fn get(&mut self) -> Result<()> {
        let Some(task_id) = self.tasks.keys().choose(&mut rng()) else {
            // Empty hashmap so skip this iteration
            return Ok(());
        };
        let payload = self.tasks.get(task_id).unwrap();

        match self.client.get(task_id).await {
            Ok(task) => {
                info!(
                    secret = task.secret,
                    operation = task.operation.to_string(),
                    id = task_id,
                    method = "GET",
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
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.patch", err, skip(self))]
    async fn patch(&mut self) -> Result<()> {
        let Some(task_id) = self.tasks.keys().choose(&mut rng()).cloned() else {
            // Empty hashmap so skip this iteration
            return Ok(());
        };
        let payload = self.tasks.get(&task_id).unwrap().clone();

        // TODO: enforce a different operation than the current one
        let random_operation = rng().random_range(0..=3);
        let operation = match random_operation {
            0 => Operation::Compute,
            1 => Operation::Merge,
            2 => Operation::Sort,
            3 => Operation::Transform,
            _ => unreachable!(),
        };

        match self.client.patch(&task_id, operation.clone()).await {
            Ok(task) => {
                self.metrics
                    .increment_total_operation(&operation.to_string());
                self.metrics
                    .increment_live_operation(&operation.to_string());
                self.metrics
                    .decrement_live_operation(&payload.operation.to_string());

                self.tasks.insert(task_id.clone(), task.clone());

                info!(
                    secret = payload.secret,
                    from_operation = payload.operation.to_string(),
                    to_operation = task.operation.to_string(),
                    id = task_id,
                    method = "PATCH",
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
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.put", err, skip(self))]
    async fn put(&mut self) -> Result<()> {
        let Some(task_id) = self.tasks.keys().choose(&mut rng()).cloned() else {
            // Empty hashmap so skip this iteration
            return Ok(());
        };
        let old_payload = self.tasks.get(&task_id).unwrap().clone();

        let new_payload = Payload::new();

        match self.client.put(&task_id, new_payload.clone()).await {
            Ok(task) => {
                self.metrics
                    .increment_total_operation(&new_payload.operation.to_string());
                self.metrics
                    .increment_live_operation(&new_payload.operation.to_string());
                self.metrics
                    .decrement_live_operation(&old_payload.operation.to_string());

                self.tasks.insert(task_id.clone(), task.clone());

                info!(
                    from_secret = old_payload.secret,
                    to_secret = new_payload.secret,
                    from_operation = old_payload.operation.to_string(),
                    to_operation = new_payload.operation.to_string(),
                    id = task_id,
                    method = "PUT",
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
                    "Failed client request"
                );
                Err(error)
            }
        }
    }

    #[instrument(name = "worker.delete", err, skip(self))]
    async fn delete(&mut self) -> Result<()> {
        let Some(task_id) = self.tasks.keys().choose(&mut rng()).cloned() else {
            // Empty hashmap so skip this iteration
            return Ok(());
        };

        match self.client.delete(&task_id).await {
            Ok(_) => {
                let payload = self.tasks.remove(&task_id).unwrap();
                self.metrics
                    .decrement_live_operation(&payload.operation.to_string());

                info!(
                    secret = payload.secret,
                    operation = payload.operation.to_string(),
                    id = task_id,
                    method = "DELETE",
                    "Deleted task"
                );
                Ok(())
            }
            Err(error) => {
                warn!(%error, task_id, method = "DELETE", "Failed client request");
                Err(error)
            }
        }
    }
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
enum Method {
    Post,
    Get,
    Patch,
    Put,
    Delete,
}

struct Metrics {
    total_operations: Counter<u64>,
    live_operations: UpDownCounter<i64>,
}

impl Metrics {
    fn new() -> Self {
        let meter = global::meter("worker");
        // Track total operations over time
        let total_operations = meter.u64_counter("total_operations").build();

        // Track the number of live tasks
        let live_operations = meter.i64_up_down_counter("live_operations").build();

        Self {
            total_operations,
            live_operations,
        }
    }

    fn increment_total_operation(&self, operation: &str) {
        self.total_operations
            .add(1, &[KeyValue::new("operation", operation.to_string())]);
    }

    fn increment_live_operation(&self, operation: &str) {
        self.live_operations
            .add(1, &[KeyValue::new("operation", operation.to_string())]);
    }

    fn decrement_live_operation(&self, operation: &str) {
        self.live_operations
            .add(-1, &[KeyValue::new("operation", operation.to_string())]);
    }
}
