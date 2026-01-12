#[cfg(feature = "archiving")]
pub mod archiving;
pub mod client;
pub mod date;
pub mod media;
pub mod parsing;

use crate::parsing::media_url::MediaUrl;
pub use async_trait;

pub const APOD_BASE_URL: &str = "https://apod.nasa.gov/apod";

#[cfg(feature = "include-html-archive")]
pub const INCLUDED_HTML_ARCHIVE: &[u8] =
    include_bytes!("../../data/apodex-html-archive-2026-01-12.bin");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApodEntry {
    pub date: date::ApodDate,
    pub title: String,
    pub explanation: String,
    pub media: MediaUrl,
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
