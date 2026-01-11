use chrono::{Local, NaiveDate};
use std::fmt::Display;

/// Counting days since 1995-6-16, where APOD starts
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApodDate(i32);

impl ApodDate {
    pub const START: Self = Self(0);
    pub const CHRONO_START: NaiveDate = NaiveDate::from_ymd_opt(1995, 6, 16).unwrap();
    /// Dates that are known to have no APOD entry
    pub const KNOWN_MISSING_DATES: [Self; 4] = [
        Self::from_ymd_unsafe(1995, 6, 17),
        Self::from_ymd_unsafe(1995, 6, 18),
        Self::from_ymd_unsafe(1995, 6, 19),
        Self::from_ymd_unsafe(2020, 6, 10),
    ];

    pub fn iter_till_today() -> impl Iterator<Item = Self> {
        (0..=Self::today().days()).filter_map(|days| {
            let date = Self(days);
            if Self::KNOWN_MISSING_DATES.contains(&date) {
                None
            } else {
                Some(date)
            }
        })
    }

    pub fn today() -> Self {
        Self::from(Local::now().date_naive())
    }

    pub fn total_apod_days() -> u32 {
        (Self::today().days() as u32)
            .saturating_add(1)
            .saturating_sub(Self::KNOWN_MISSING_DATES.len() as u32)
    }

    pub fn parse_from_str(date: &str, fmt: &str) -> Option<Self> {
        NaiveDate::parse_from_str(date, fmt).ok().map(Self::from)
    }

    pub fn days(&self) -> i32 {
        self.0
    }

    pub fn format(&self, fmt: &str) -> String {
        NaiveDate::from(*self).format(fmt).to_string()
    }

    pub fn link(&self) -> Option<String> {
        if self.days() < 0 {
            return None;
        }
        Some(format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            self.format("%y%m%d")
        ))
    }

    pub fn inc(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self::from)
    }

    /// Will blindly assume that the date is valid, allows it to be a constant expression
    pub const fn from_ymd_unsafe(year: i32, month: u32, day: u32) -> Self {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let days_since = date.signed_duration_since(Self::CHRONO_START).num_days() as i32;
        Self(days_since)
    }
}

impl From<NaiveDate> for ApodDate {
    fn from(date: NaiveDate) -> Self {
        let days_since = date.signed_duration_since(Self::CHRONO_START).num_days() as i32;
        Self(days_since)
    }
}

impl From<ApodDate> for NaiveDate {
    fn from(value: ApodDate) -> Self {
        ApodDate::CHRONO_START + chrono::Duration::days(value.0 as i64)
    }
}

impl Display for ApodDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format("%Y-%m-%d"))
    }
}
