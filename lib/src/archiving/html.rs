use crate::archiving::ArchiveEntry;
use crate::date::ApodDate;

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode)]
pub struct ArchiveHtml {
    pub date: ApodDate,
    pub html: String,
}

impl ArchiveHtml {
    pub fn new(date: ApodDate, html: String) -> Self {
        Self { date, html }
    }
}

impl ArchiveEntry for ArchiveHtml {
    fn date(&self) -> ApodDate {
        self.date
    }
}
