mod client;
mod payload;

use crate::{
    client::Client,
    payload::{Operation, Payload},
};
use eyre::Result;
use rand::{RngExt, rng, seq::IteratorRandom};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber so we can observe the logs in case something goes wrong.
    // At this stage we do not care about logging as there will be no dashboards to view them.
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: This needs to be changed to contain many server URLs
    // Docker compose will resolve the URL and we'll use it to send payloads to the server.
    let url = std::env::var("SERVER_URL")?;

    // Dumb client that only sends created payloads
    let client = Client::new(url);

    // Track tasks so we can use other methods
    let mut tasks = HashMap::new();

    // We need the generator to run indefinitely without too much spam in this stage so we'll send 1
    // type of payload at a fixed rate.
    loop {
        // Generate a number to determine which type of request to send
        let random_request_type = rng().random_range(0..=99);

        match random_request_type {
            // POST 40% of the time
            0..=39 => {
                let payload = Payload::new();

                match client.post(&payload).await {
                    Ok(id) => {
                        tasks.insert(id.clone(), payload.clone());
                        info!(
                            secret = payload.secret,
                            operation = payload.operation.to_string(),
                            id,
                            method = "POST",
                            "Stored task"
                        );
                    }
                    Err(error) => warn!(
                        %error,
                        secret = payload.secret,
                        operation = payload.operation.to_string(),
                        method = "POST",
                        "Failed client request"
                    ),
                }
            }
            // GET 25% of the time
            40..=64 => {
                let Some(task_id) = tasks.keys().choose(&mut rng()) else {
                    // Empty hashmap so skip this iteration
                    continue;
                };
                let payload = tasks.get(task_id).unwrap();

                match client.get(task_id).await {
                    Ok(task) => {
                        info!(
                            secret = task.secret,
                            operation = task.operation.to_string(),
                            id = task_id,
                            method = "GET",
                            "Retrieved task"
                        );
                    }
                    Err(error) => warn!(
                        %error,
                        secret = payload.secret,
                        operation = payload.operation.to_string(),
                        id = task_id,
                        method = "GET",
                        "Failed client request"
                    ),
                }
            }
            // PATCH 15% of the time
            65..=79 => {
                let Some(task_id) = tasks.keys().choose(&mut rng()).cloned() else {
                    // Empty hashmap so skip this iteration
                    continue;
                };
                let payload = tasks.get(&task_id).unwrap().clone();

                let random_operation = rng().random_range(0..=3);
                let operation = match random_operation {
                    0 => Operation::Compute,
                    1 => Operation::Merge,
                    2 => Operation::Sort,
                    3 => Operation::Transform,
                    _ => unreachable!(),
                };

                match client.patch(&task_id, operation.clone()).await {
                    Ok(task) => {
                        tasks.insert(task_id.clone(), task.clone());
                        info!(
                            secret = payload.secret,
                            from_operation = payload.operation.to_string(),
                            to_operation = task.operation.to_string(),
                            id = task_id,
                            method = "PATCH",
                            "Patched task"
                        );
                    }
                    Err(error) => warn!(
                        %error,
                        secret = payload.secret,
                        from_operation = payload.operation.to_string(),
                        to_operation = operation.to_string(),
                        id = task_id,
                        method = "PATCH",
                        "Failed client request"
                    ),
                }
            }
            // PUT 10% of the time
            80..=89 => {
                let Some(task_id) = tasks.keys().choose(&mut rng()).cloned() else {
                    // Empty hashmap so skip this iteration
                    continue;
                };
                let old_payload = tasks.get(&task_id).unwrap().clone();

                let new_payload = Payload::new();

                match client.put(&task_id, new_payload.clone()).await {
                    Ok(task) => {
                        tasks.insert(task_id.clone(), task.clone());
                        info!(
                            from_secret = old_payload.secret,
                            to_secret = new_payload.secret,
                            from_operation = old_payload.operation.to_string(),
                            to_operation = new_payload.operation.to_string(),
                            id = task_id,
                            method = "PUT",
                            "Put task"
                        );
                    }
                    Err(error) => warn!(
                        %error,
                        from_secret = old_payload.secret,
                        to_secret = new_payload.secret,
                        from_operation = old_payload.operation.to_string(),
                        to_operation = new_payload.operation.to_string(),
                        id = task_id,
                        method = "PUT",
                        "Failed client request"
                    ),
                }
            }
            // DELETE 10% of the time
            90..=99 => {
                let Some(task_id) = tasks.keys().choose(&mut rng()).cloned() else {
                    // Empty hashmap so skip this iteration
                    continue;
                };

                match client.delete(&task_id).await {
                    Ok(_) => {
                        let payload = tasks.remove(&task_id).unwrap();
                        info!(
                            secret = payload.secret,
                            operation = payload.operation.to_string(),
                            id = task_id,
                            method = "DELETE",
                            "Deleted task"
                        );
                    }
                    Err(error) => {
                        warn!(%error, task_id, method = "DELETE", "Failed client request")
                    }
                }
            }
            _ => unreachable!(),
        }

        sleep(Duration::from_millis(10)).await;
    }
}
