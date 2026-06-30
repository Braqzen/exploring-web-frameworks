use crate::payload::Payload;
use rand::{
    rng,
    seq::{IndexedRandom, IteratorRandom},
};
use serde::Deserialize;
use std::collections::HashMap;

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

        Self { apis }
    }

    pub fn select(&self) -> (Provider, String, Method) {
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

/// Distribution mapping the method to a weight percentage for random selection.
///
/// I.e. call this method X% of the time.
const REQUESTS: [(Method, u32); 6] = [
    (Method::Post, 30),   // 30%
    (Method::Get, 25),    // 25%
    (Method::Patch, 15),  // 15%
    (Method::Put, 10),    // 10%
    (Method::Delete, 15), // 10%
    (Method::Head, 5),    // 5%
];

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
