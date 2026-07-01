use crate::payload::Payload;
use rand::{
    RngExt, rng,
    seq::{IndexedRandom, IteratorRandom, SliceRandom},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Axum,
    Actix,
    Warp,
    Rocket,
    Poem,
    Salvo,
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Axum => "axum".to_string(),
            Provider::Actix => "actix".to_string(),
            Provider::Warp => "warp".to_string(),
            Provider::Rocket => "rocket".to_string(),
            Provider::Poem => "poem".to_string(),
            Provider::Salvo => "salvo".to_string(),
        }
    }
}

pub struct ApiManager {
    apis: HashMap<Provider, ApiState>,
    requests: Arc<Mutex<[(Method, u16); 6]>>,
}

impl ApiManager {
    pub fn new(apis: HashMap<Provider, String>) -> Self {
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

        // Randomly set the requests for the first time
        let random_requests = randomise_requests();

        // Enable background mutation of requests
        let requests = Arc::new(Mutex::new(random_requests));

        // Sprinkle in the fun part of rust to make the compiler happy
        let task_requests = Arc::clone(&requests);

        // Spawn a background task (not a thread) which periodically randomises the request distribution
        tokio::spawn(async move {
            loop {
                random_sleep().await;

                {
                    let requests = randomise_requests();

                    match task_requests.lock() {
                        Ok(mut guard) => {
                            *guard = requests;
                        }
                        Err(error) => {
                            error!(%error, "Poisoned request lock in background randomiser task of ApiManager");

                            // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                            let mut guard = error.into_inner();
                            *guard = requests;
                            task_requests.clear_poison();
                        }
                    }
                }
            }
        });

        Self { apis, requests }
    }

    pub fn select(&self) -> (Provider, String, Method) {
        // SAFETY: We hold exlusive access to apis and it's non-empty therefore cannot panic
        let (provider, state) = self.apis.iter().choose(&mut rng()).unwrap();

        // We need at least 1 task to have an ID to use to send requests
        // If no tasks, default to POST to get first ID
        if state.tasks.is_empty() {
            (provider.clone(), state.url.clone(), Method::Post)
        } else {
            // SAFETY: REQUESTS is non-empty and has values greater than 0
            let guard = match self.requests.lock() {
                Ok(guard) => guard,
                Err(error) => {
                    error!(%error, "Poisoned request lock in select method of ApiManager");

                    // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                    let requests = randomise_requests();

                    let mut guard = error.into_inner();
                    *guard = requests;
                    self.requests.clear_poison();

                    guard
                }
            };

            let (method, _) = guard
                .choose_weighted(&mut rng(), |(_, weight)| *weight)
                .unwrap();

            (provider.clone(), state.url.clone(), method.to_owned())
        }
    }

    pub fn insert(&mut self, provider: &Provider, id: &String, payload: &Payload) -> Option<()> {
        self.apis
            .get_mut(provider)?
            .tasks
            .insert(id.clone(), payload.clone());
        Some(())
    }

    pub fn remove(&mut self, provider: &Provider, task_id: &String) -> Option<Payload> {
        self.apis.get_mut(provider)?.tasks.remove(task_id)
    }

    pub fn payload(&self, provider: &Provider) -> Option<(String, Payload)> {
        let (task_id, payload) = self.apis.get(provider)?.tasks.iter().choose(&mut rng())?;

        Some((task_id.clone(), payload.clone()))
    }
}

struct ApiState {
    /// URL of the container to send a request to
    url: String,
    /// Task ID -> Payload
    tasks: HashMap<String, Payload>,
}

/// The method of the HTTP request to send to a server.
#[derive(Clone, PartialEq)]
pub enum Method {
    Post,
    Get,
    Patch,
    Put,
    Delete,
    Head,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Patch => "PATCH",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
        }
        .to_string()
    }
}

/// Randomly create a distribution of methods to send to a server.
///
/// I.e. call this method X% of the time.
///
/// The call selection assumes that at least 1 method is non-zero so return None if all were set to 0.
fn randomiser() -> Option<[(Method, u16); 6]> {
    // Select a random number to use to spread out the method weights
    let mut tokens: u16 = 1000;

    // Track if at least 1 method is non-zero
    let mut non_zero = false;

    let mut requests = [
        (Method::Post, 0),
        (Method::Get, 0),
        (Method::Patch, 0),
        (Method::Put, 0),
        (Method::Delete, 0),
        (Method::Head, 0),
    ];

    requests.shuffle(&mut rng());

    // For the sake of logging (and reducing performance) track each value
    let mut post = 0;
    let mut get = 0;
    let mut patch = 0;
    let mut put = 0;
    let mut delete = 0;
    let mut head = 0;

    // Randomly set a weight for each method
    requests.iter_mut().for_each(|(method, weight)| {
        let random_weight = rng().random_range(0..=tokens);

        tokens -= random_weight;
        *weight = random_weight;

        match method {
            Method::Post => post = random_weight,
            Method::Get => get = random_weight,
            Method::Patch => patch = random_weight,
            Method::Put => put = random_weight,
            Method::Delete => delete = random_weight,
            Method::Head => head = random_weight,
        }

        if 0 < random_weight {
            non_zero = true;
        }
    });

    if non_zero {
        let total = post + get + patch + put + delete + head;
        info!(
            post = ((post as f64 / total as f64) * 1000.0).round() / 10.0,
            get = ((get as f64 / total as f64) * 1000.0).round() / 10.0,
            patch = ((patch as f64 / total as f64) * 1000.0).round() / 10.0,
            put = ((put as f64 / total as f64) * 1000.0).round() / 10.0,
            delete = ((delete as f64 / total as f64) * 1000.0).round() / 10.0,
            head = ((head as f64 / total as f64) * 1000.0).round() / 10.0,
            total,
            "Randomised request distribution"
        );
        Some(requests)
    } else {
        None
    }
}

fn randomise_requests() -> [(Method, u16); 6] {
    let mut requests = randomiser();

    // Realistically this will never trigger and if it does it likely won't loop a 2nd time
    while requests.is_none() {
        requests = randomiser();
    }

    requests.unwrap()
}

async fn random_sleep() {
    const MILLISECONDS: u64 = 1000;

    // Randomly sleep for 5 to 15 seconds after each randomisation to buffer updates
    let sleep_duration = rng().random_range(5 * MILLISECONDS..=15 * MILLISECONDS);

    sleep(Duration::from_millis(sleep_duration)).await;
}
