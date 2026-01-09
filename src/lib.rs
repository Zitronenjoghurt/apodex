#[cfg(feature = "archiving")]
pub mod archiving;
pub mod client;
pub mod parsing;
#[cfg(feature = "scraper")]
pub mod scraper;

pub use async_trait;
pub use chrono;
#[cfg(feature = "futures")]
pub use futures;

pub struct ApodEntry {
    pub title: String,
}
