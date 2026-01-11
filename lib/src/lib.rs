#[cfg(feature = "archiving")]
pub mod archiving;
pub mod client;
pub mod parsing;
#[cfg(feature = "scraping")]
pub mod scraping;

pub use async_trait;
pub use chrono;
use chrono::{Local, NaiveDate};
#[cfg(feature = "futures")]
pub use futures;

pub const APOD_START_DATE: NaiveDate = NaiveDate::from_ymd_opt(1995, 6, 16).unwrap();

pub fn iter_apod_dates() -> impl Iterator<Item = NaiveDate> {
    let today = Local::now().date_naive();
    APOD_START_DATE
        .iter_days()
        .take_while(move |&date| date <= today)
}

pub fn days_since_apod_start() -> i64 {
    let today = Local::now().date_naive();
    today.signed_duration_since(APOD_START_DATE).num_days() + 1
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ApodEntry {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub title: String,
}

impl ApodEntry {
    pub fn date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32)
    }

    pub fn link(&self) -> Option<String> {
        let date = self.date()?;
        Some(format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            date.format("%y%m%d")
        ))
    }
}

#[cfg(feature = "archiving")]
impl archiving::ArchiveEntry for ApodEntry {
    fn naive_date(&self) -> Option<NaiveDate> {
        self.date()
    }
}
