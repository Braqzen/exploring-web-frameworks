//! The client is used to send payload requests to the server.

use crate::payload::{Operation, Payload};
use eyre::Result;
use reqwest::Client as ReqwestClient;
use serde_json::json;

pub struct Client {
    url: String,
    client: ReqwestClient,
}

impl Client {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: ReqwestClient::new(),
        }
    }

    pub async fn post(&self, payload: &Payload) -> Result<String> {
        let response = self
            .client
            .post(&self.url)
            .json(payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<String>().await?)
    }

    pub async fn get(&self, task_id: &str) -> Result<Payload> {
        let url = self.task_url(task_id);

        let response = self.client.get(&url).send().await?.error_for_status()?;

        Ok(response.json::<Payload>().await?)
    }

    pub async fn patch(&self, task_id: &str, operation: Operation) -> Result<Payload> {
        let url = self.task_url(task_id);

        let response = self
            .client
            .patch(&url)
            .json(&json!({ "operation": operation }))
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<Payload>().await?)
    }

    pub async fn put(&self, task_id: &str, payload: Payload) -> Result<Payload> {
        let url = self.task_url(task_id);

        let response = self
            .client
            .put(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json::<Payload>().await?)
    }

    pub async fn delete(&self, task_id: &str) -> Result<()> {
        let url = self.task_url(task_id);

        self.client.delete(&url).send().await?.error_for_status()?;

        Ok(())
    }

    fn task_url(&self, task_id: &str) -> String {
        format!("{}/{}", self.url.trim_end_matches('/'), task_id)
    }
}
