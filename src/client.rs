#[cfg(feature = "reqwest-client")]
pub mod reqwest;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to fetch URL '{url}': {source}")]
    Fetch {
        source: Box<dyn std::error::Error + Send + Sync>,
        url: String,
    },
}

#[async_trait::async_trait]
pub trait ApodClient {
    async fn fetch(
        &self,
        url: &str,
    ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>>;

    async fn fetch_page(&self, date: chrono::NaiveDate) -> Result<Option<String>, ClientError> {
        let url = format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            date.format("%y%m%d")
        );

        let Some(bytes) = self
            .fetch(&url)
            .await
            .map_err(|e| ClientError::Fetch { url, source: e })?
        else {
            return Ok(None);
        };

        Ok(Some(String::from_utf8_lossy(bytes.as_slice()).into_owned()))
    }
}
