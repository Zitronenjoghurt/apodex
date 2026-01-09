use crate::client::ApodClient;
use std::error::Error;

pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ApodClient for ReqwestClient {
    async fn fetch(&self, url: &str) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>> {
        let response = self
            .client
            .get(url)
            .header(
                "User-Agent",
                format!("apodex/{} (APOD archiving tool)", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        };

        let response = response.error_for_status()?;
        Ok(Some(response.bytes().await?.to_vec()))
    }
}
