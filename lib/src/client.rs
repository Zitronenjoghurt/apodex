use crate::date::ApodDate;
use crate::media::{MediaEntry, MediaType};
use crate::{ApodEntry, APOD_BASE_URL};

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

    async fn fetch_page(&self, date: ApodDate) -> Result<Option<String>, ClientError> {
        let url = format!("{APOD_BASE_URL}/ap{}.html", date.format("%y%m%d"));

        let Some(bytes) = self
            .fetch(&url)
            .await
            .map_err(|e| ClientError::Fetch { url, source: e })?
        else {
            return Ok(None);
        };

        Ok(Some(String::from_utf8_lossy(bytes.as_slice()).into_owned()))
    }

    async fn fetch_media(&self, entry: &ApodEntry) -> Result<Option<MediaEntry>, ClientError> {
        let Some(url) = entry.media.highest_quality() else {
            return Ok(None);
        };

        let Some(bytes) = self.fetch(url).await.map_err(|e| ClientError::Fetch {
            url: url.to_owned(),
            source: e,
        })?
        else {
            return Ok(None);
        };

        Ok(Some(MediaEntry {
            media_type: MediaType::ImagePNG,
            data: bytes,
        }))
    }
}
