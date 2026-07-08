use crate::{
    methods::method::Method,
    randomiser::Randomiser,
    settings::{ProviderName, ProviderOptions},
};
use rand::{rng, seq::IndexedRandom};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::error;

pub struct MethodManager {
    requests: Arc<Mutex<HashMap<ProviderName, Vec<(Method, u16)>>>>,
}

impl MethodManager {
    pub fn new(api: Vec<ProviderOptions>) -> Self {
        let requests: HashMap<ProviderName, Vec<(Method, u16)>> = api
            .into_iter()
            .map(|options| {
                (
                    options.provider.clone(),
                    Randomiser::requests(&options.provider, &options.methods),
                )
            })
            .collect();

        // Enable background mutation of requests
        let requests = Arc::new(Mutex::new(requests));

        // Sprinkle in the fun part of rust to make the compiler happy
        let task_requests = Arc::clone(&requests);

        // Spawn a background task (not a thread) which periodically randomises the request distribution
        tokio::spawn(async move {
            loop {
                Randomiser::sleep().await;

                {
                    match task_requests.lock() {
                        Ok(mut guard) => {
                            let mut new_requests = HashMap::new();
                            for (provider, methods) in guard.iter() {
                                new_requests.insert(
                                    provider.clone(),
                                    Randomiser::requests(
                                        provider,
                                        &methods
                                            .iter()
                                            .map(|(method, _)| method.clone())
                                            .collect::<Vec<_>>(),
                                    ),
                                );
                            }
                            *guard = new_requests;
                        }
                        Err(error) => {
                            error!(%error, "Poisoned request lock in background randomiser task of MethodManager");

                            // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                            let mut guard = error.into_inner();

                            let mut new_requests = HashMap::new();
                            for (provider, methods) in guard.iter() {
                                new_requests.insert(
                                    provider.clone(),
                                    Randomiser::requests(
                                        provider,
                                        &methods
                                            .iter()
                                            .map(|(method, _)| method.clone())
                                            .collect::<Vec<_>>(),
                                    ),
                                );
                            }
                            *guard = new_requests;
                            task_requests.clear_poison();
                        }
                    }
                }
            }
        });

        Self { requests }
    }

    pub fn select(&self, provider: ProviderName, post: bool) -> Method {
        if post {
            Method::Post
        } else {
            let guard = match self.requests.lock() {
                Ok(guard) => guard,
                Err(error) => {
                    error!(%error, "Poisoned request lock in select method of ApiManager");

                    let mut guard = error.into_inner();

                    // Requests is non-sensitive so recovery is just wiping the data with a new distribution
                    let mut new_requests = HashMap::new();
                    for (provider, methods) in guard.iter() {
                        new_requests.insert(
                            provider.clone(),
                            Randomiser::requests(
                                provider,
                                &methods
                                    .iter()
                                    .map(|(method, _)| method.clone())
                                    .collect::<Vec<_>>(),
                            ),
                        );
                    }
                    *guard = new_requests;
                    self.requests.clear_poison();

                    guard
                }
            };

            // TODO: unsafe if caller passes a disabled provider
            let provider_methods = guard.get(&provider).unwrap();

            // SAFETY: REQUESTS is non-empty and has values greater than 0
            let (method, _) = provider_methods
                .choose_weighted(&mut rng(), |(_, weight)| *weight)
                .unwrap();

            method.clone()
        }
    }
}
