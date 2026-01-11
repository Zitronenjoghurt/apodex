use crate::client::ApodClient;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

pub use leaky_bucket;
use leaky_bucket::RateLimiter;

#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
}

impl ReqwestClient {
    pub fn new(rate_limiter: RateLimiter) -> Self {
        Self {
            rate_limiter: Arc::new(rate_limiter),
            ..Default::default()
        }
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            // From my experience, this is the most reasonable rate limiter for APOD entries
            rate_limiter: Arc::new(
                leaky_bucket::Builder::default()
                    .refill(1)
                    .interval(Duration::from_secs(2))
                    .max(1)
                    .build(),
            ),
        }
    }
}

#[async_trait::async_trait]
impl ApodClient for ReqwestClient {
    async fn fetch(&self, url: &str) -> Result<Option<Vec<u8>>, Box<dyn Error + Send + Sync>> {
        self.rate_limiter.acquire_one().await;

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
