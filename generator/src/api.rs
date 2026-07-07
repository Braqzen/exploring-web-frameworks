use crate::{
    config::{Language, ProviderName, ProviderOptions},
    payload::Payload,
    provider::Provider,
};
use rand::{rng, seq::IteratorRandom};
use std::collections::HashMap;

pub struct ApiManager {
    apis: HashMap<ProviderName, ApiState>,
}

impl ApiManager {
    pub fn new(apis: Vec<ProviderOptions>) -> Self {
        let apis = apis
            .into_iter()
            .map(|options| {
                (
                    options.provider.clone(),
                    ApiState {
                        url: options.url.clone(),
                        tasks: HashMap::new(),
                        language: options.language,
                    },
                )
            })
            .collect();

        Self { apis }
    }

    pub fn select(&self) -> (Provider, bool) {
        // TOD0: annoying post: bool return. This type should not have coupled functionality with the MethodManager
        //       it should only return (provider, url).
        // TODO: randomise provider distribution similar to requests/operations
        // SAFETY: We hold exlusive access to apis and it's non-empty therefore cannot panic
        let (provider, state) = self.apis.iter().choose(&mut rng()).unwrap();

        // We need at least 1 task to have an ID to use to send requests
        // If no tasks, default to POST to get first ID
        let post = if state.tasks.is_empty() { true } else { false };

        (
            Provider::new(provider.clone(), state.url.clone(), state.language.clone()),
            post,
        )
    }

    pub fn insert(
        &mut self,
        provider: &ProviderName,
        id: &String,
        payload: &Payload,
    ) -> Option<()> {
        self.apis
            .get_mut(provider)?
            .tasks
            .insert(id.clone(), payload.clone());
        Some(())
    }

    pub fn remove(&mut self, provider: &ProviderName, task_id: &String) -> Option<Payload> {
        self.apis.get_mut(provider)?.tasks.remove(task_id)
    }

    pub fn payload(&self, provider: &ProviderName) -> Option<(String, Payload)> {
        let (task_id, payload) = self.apis.get(provider)?.tasks.iter().choose(&mut rng())?;

        Some((task_id.clone(), payload.clone()))
    }
}

struct ApiState {
    /// URL of the container to send a request to
    url: String,
    /// Task ID -> Payload
    tasks: HashMap<String, Payload>,
    // TODO: Hack this should not be on here
    /// Programming Language
    language: Language,
}
