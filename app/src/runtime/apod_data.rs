use crate::runtime::task::{TaskContext, TaskHandler};
use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::{Archive, ArchiveError};
use apodex::date::ApodDate;
use apodex::parsing::quality_control::QualityWarning;
use apodex::parsing::ParseError;
use apodex::ApodEntry;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

struct LoadedHtmlArchive {
    html_archive: Archive<ArchiveHtml>,
    entry_archive: Archive<ApodEntry>,
    parse_warnings: HashMap<ApodDate, HashSet<QualityWarning>>,
    parse_errors: HashMap<ApodDate, ParseError>,
}

pub struct ApodData {
    last_update: Instant,
    html_archive: Archive<ArchiveHtml>,
    entry_archive: Archive<ApodEntry>,
    parse_warnings: HashMap<ApodDate, HashSet<QualityWarning>>,
    parse_errors: HashMap<ApodDate, ParseError>,
    load_html_task: TaskHandler<Result<LoadedHtmlArchive, ArchiveError>>,
    save_html_task: TaskHandler<Result<(), ArchiveError>>,
}

impl Default for ApodData {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            html_archive: Archive::default(),
            entry_archive: Archive::default(),
            parse_warnings: HashMap::new(),
            parse_errors: HashMap::new(),
            load_html_task: TaskHandler::default(),
            save_html_task: TaskHandler::default(),
        }
    }
}

impl ApodData {
    pub fn start_load_included_html(&mut self, handle: &tokio::runtime::Handle) {
        self.load_html_task.spawn(handle, |ctx| async move {
            ctx.set_status("Loading HTML archive...");
            let html_archive: Archive<ArchiveHtml> = Archive::load_included_html_archive();
            Self::load_html(ctx, html_archive)
        });
    }

    pub fn start_load_html(&mut self, handle: &tokio::runtime::Handle, path: impl AsRef<Path>) {
        let path = path.as_ref().to_owned();
        self.load_html_task.spawn(handle, |ctx| async move {
            ctx.set_status("Loading HTML archive...");
            let html_archive: Archive<ArchiveHtml> = Archive::load(&path)?;
            Self::load_html(ctx, html_archive)
        });
    }

    fn load_html(
        ctx: TaskContext,
        archive: Archive<ArchiveHtml>,
    ) -> Result<LoadedHtmlArchive, ArchiveError> {
        let mut entry_archive = Archive::default();
        let mut parse_warnings = HashMap::new();
        let mut parse_errors = HashMap::new();

        ctx.set_status("Parsing HTML archive...");
        for (i, (date, entry)) in archive.iter().enumerate() {
            let result = apodex::parsing::verbose::parse_html_verbose(*date, entry.html.as_str());

            if let Some(entry) = result.entry {
                entry_archive.push(entry);
            }

            if !result.warnings.is_empty() {
                parse_warnings.insert(*date, result.warnings);
            }

            if let Some(error) = result.error {
                parse_errors.insert(*date, error);
            }

            ctx.set_status(format!(
                "Parsing HTML archive... ({:03}/{:03})",
                i + 1,
                archive.len()
            ))
        }

        Ok(LoadedHtmlArchive {
            html_archive: archive,
            entry_archive,
            parse_warnings,
            parse_errors,
        })
    }

    pub fn poll_load_html(&mut self) -> Option<Result<(), ArchiveError>> {
        let result = self.load_html_task.poll()?;
        Some(result.map(|loaded| {
            self.html_archive = loaded.html_archive;
            self.entry_archive = loaded.entry_archive;
            self.parse_warnings = loaded.parse_warnings;
            self.parse_errors = loaded.parse_errors;
            self.last_update = Instant::now();
        }))
    }

    pub fn start_save_html(&mut self, handle: &tokio::runtime::Handle, path: impl AsRef<Path>) {
        let path = path.as_ref().to_owned();
        let archive = self.html_archive.clone();
        self.save_html_task.spawn(handle, async move |ctx| {
            ctx.set_status("Compressing as small as possible, this might take a bit...");
            archive.save(&path, 22)?;
            Ok(())
        });
    }

    pub fn poll_save_html(&mut self) -> Option<Result<(), ArchiveError>> {
        self.save_html_task.poll()
    }

    pub fn load_busy(&self) -> bool {
        self.load_html_task.is_busy()
    }

    pub fn load_status(&self) -> Option<String> {
        self.load_html_task.status()
    }

    pub fn save_busy(&self) -> bool {
        self.save_html_task.is_busy()
    }

    pub fn save_status(&self) -> Option<String> {
        self.save_html_task.status()
    }

    pub fn get_html(&self, date: ApodDate) -> Option<&ArchiveHtml> {
        self.html_archive.get(date)
    }

    pub fn get_entry(&self, date: ApodDate) -> Option<&ApodEntry> {
        self.entry_archive.get(date)
    }

    pub fn get_warnings(&self, date: ApodDate) -> Option<&HashSet<QualityWarning>> {
        self.parse_warnings.get(&date)
    }

    pub fn get_error(&self, date: ApodDate) -> Option<&ParseError> {
        self.parse_errors.get(&date)
    }

    pub fn last_update(&self) -> Instant {
        self.last_update
    }

    pub fn latest_html_date(&self) -> Option<ApodDate> {
        self.html_archive.latest_date()
    }

    pub fn latest_entry_date(&self) -> Option<ApodDate> {
        self.entry_archive.latest_date()
    }
}
