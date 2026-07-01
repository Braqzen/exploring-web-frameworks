use crate::randomiser::Randomiser;
use rand::{rng, seq::IndexedRandom};
use std::sync::{Arc, Mutex};
use tracing::error;

pub struct MethodManager {
    requests: Arc<Mutex<[(Method, u16); 6]>>,
}

impl MethodManager {
    pub fn new() -> Self {
        // Randomly set the requests for the first time
        let random_requests = Randomiser::requests();

        // Enable background mutation of requests
        let requests = Arc::new(Mutex::new(random_requests));

        // Sprinkle in the fun part of rust to make the compiler happy
        let task_requests = Arc::clone(&requests);

        // Spawn a background task (not a thread) which periodically randomises the request distribution
        tokio::spawn(async move {
            loop {
                Randomiser::sleep().await;

                {
                    let requests = Randomiser::requests();

                    match task_requests.lock() {
                        Ok(mut guard) => {
                            *guard = requests;
                        }
                        Err(error) => {
                            error!(%error, "Poisoned request lock in background randomiser task of MethodManager");

                            // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                            let mut guard = error.into_inner();
                            *guard = requests;
                            task_requests.clear_poison();
                        }
                    }
                }
            }
        });

        Self { requests }
    }

    pub fn select(&self, post: bool) -> Method {
        if post {
            Method::Post
        } else {
            let guard = match self.requests.lock() {
                Ok(guard) => guard,
                Err(error) => {
                    error!(%error, "Poisoned request lock in select method of ApiManager");

                    // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                    let requests = Randomiser::requests();

                    let mut guard = error.into_inner();
                    *guard = requests;
                    self.requests.clear_poison();

                    guard
                }
            };

            // SAFETY: REQUESTS is non-empty and has values greater than 0
            let (method, _) = guard
                .choose_weighted(&mut rng(), |(_, weight)| *weight)
                .unwrap();

            method.clone()
        }
    }
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
