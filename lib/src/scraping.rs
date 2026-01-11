use crate::client::{ApodClient, ClientError};
use crate::date::ApodDate;
use std::pin::Pin;
use std::time::Duration;

pub struct Scraper<C: ApodClient + Sync> {
    client: C,
    delay: Duration,
}

impl<C: ApodClient + Sync> Scraper<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            delay: Duration::from_secs(1),
        }
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub async fn fetch_html(&self, date: ApodDate) -> Result<Option<String>, ClientError> {
        let page = self.client.fetch_page(date).await?;
        tokio::time::sleep(self.delay).await;
        Ok(page)
    }

    pub fn iter_html(
        &self,
        start: ApodDate,
        end: ApodDate,
    ) -> Pin<
        Box<
            dyn futures::Stream<Item = Result<(ApodDate, String), (ApodDate, ClientError)>>
                + Send
                + '_,
        >,
    > {
        Box::pin(async_stream::stream! {
            let mut current = start;
            while current <= end {
                match self.fetch_html(current).await {
                    Ok(Some(html)) => yield Ok((current, html)),
                    Ok(None) => {},
                    Err(e) => yield Err((current, e)),
                }
                current.inc();
            }
        })
    }
}
