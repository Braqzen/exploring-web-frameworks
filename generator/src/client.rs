//! The client is used to send payload requests to the server.

use crate::payload::Payload;
use eyre::Result;
use reqwest::Client as ReqwestClient;

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
        // In the initial stage we do not care whether the payload has succeeded as that has no impact
        // on the game except that it stops and the server must be fixed.
        // Perhaps in a future implementation the server may have a fixed size queue with backpressure
        // and it may be interesting to implement some additional complexity into the generator but for
        // now we fire and forget.
        let response = self.client.post(&self.url).json(payload).send().await?;
        Ok(response.json::<String>().await?)
    }
}
