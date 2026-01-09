#[cfg(feature = "archiving")]
pub mod archiving;
pub mod client;
pub mod parsing;
#[cfg(feature = "scraping")]
pub mod scraping;

pub use async_trait;
pub use chrono;
#[cfg(feature = "futures")]
pub use futures;

#[derive(Debug)]
pub struct ApodEntry {
    pub date: chrono::NaiveDate,
    pub title: String,
}

impl ApodEntry {
    pub fn link(&self) -> String {
        format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            self.date.format("%y%m%d")
        )
    }
}
