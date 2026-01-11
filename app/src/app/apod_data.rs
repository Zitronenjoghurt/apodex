use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::Archive;
use apodex::chrono::NaiveDate;
use apodex::parsing::quality_control::QualityWarning;
use apodex::parsing::ParseError;
use apodex::ApodEntry;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

pub struct ApodData {
    last_update: Instant,
    html_archive: Archive<ArchiveHtml>,
    entry_archive: Archive<ApodEntry>,
    parse_warnings: HashMap<NaiveDate, HashSet<QualityWarning>>,
    parse_errors: HashMap<NaiveDate, ParseError>,
}

impl Default for ApodData {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            html_archive: Archive::default(),
            entry_archive: Archive::default(),
            parse_warnings: HashMap::new(),
            parse_errors: HashMap::new(),
        }
    }
}

impl ApodData {
    pub fn load_html_archive(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.html_archive = Archive::load(path.as_ref())?;
        self.parse_html_archive();
        self.last_update = Instant::now();
        Ok(())
    }

    pub fn load_entry_archive(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.entry_archive = Archive::load(path.as_ref())?;
        self.html_archive.clear();
        self.parse_warnings.clear();
        self.parse_errors.clear();
        self.last_update = Instant::now();
        Ok(())
    }

    pub fn parse_html_archive(&mut self) {
        self.entry_archive.clear();
        self.parse_warnings.clear();
        self.parse_errors.clear();
        for (date, entry) in self.html_archive.iter() {
            let verbose_result =
                apodex::parsing::verbose::parse_html_verbose(*date, entry.html.as_str());
            if let Some(entry) = verbose_result.entry {
                self.entry_archive.push(entry);
            }
            if !verbose_result.warnings.is_empty() {
                self.parse_warnings.insert(*date, verbose_result.warnings);
            }
            if let Some(error) = verbose_result.error {
                self.parse_errors.insert(*date, error);
            }
        }
    }

    pub fn get_html(&self, date: NaiveDate) -> Option<&ArchiveHtml> {
        self.html_archive.get(date)
    }

    pub fn get_entry(&self, date: NaiveDate) -> Option<&ApodEntry> {
        self.entry_archive.get(date)
    }

    pub fn get_warnings(&self, date: NaiveDate) -> Option<&HashSet<QualityWarning>> {
        self.parse_warnings.get(&date)
    }

    pub fn get_error(&self, date: NaiveDate) -> Option<&ParseError> {
        self.parse_errors.get(&date)
    }

    pub fn last_update(&self) -> Instant {
        self.last_update
    }
}
