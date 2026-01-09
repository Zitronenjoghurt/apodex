use crate::archiving::ArchiveEntry;
use chrono::NaiveDate;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct ArchiveHtml {
    pub date: String,
    pub html: String,
}

impl ArchiveHtml {
    pub fn new(date: NaiveDate, html: String) -> Self {
        Self {
            date: date.format("%Y-%m-%d").to_string(),
            html,
        }
    }
}

impl ArchiveEntry for ArchiveHtml {
    fn naive_date(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").ok()
    }
}
