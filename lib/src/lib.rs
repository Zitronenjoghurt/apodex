#[cfg(feature = "archiving")]
pub mod archiving;
pub mod client;
pub mod date;
pub mod parsing;
#[cfg(feature = "scraping")]
pub mod scraping;

pub use async_trait;
#[cfg(feature = "futures")]
pub use futures;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ApodEntry {
    pub date: date::ApodDate,
    pub title: String,
}

impl ApodEntry {
    pub fn link(&self) -> Option<String> {
        self.date.link()
    }
}

#[cfg(feature = "archiving")]
impl archiving::ArchiveEntry for ApodEntry {
    fn date(&self) -> date::ApodDate {
        self.date
    }
}
