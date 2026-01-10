use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::Archive;
use apodex::ApodEntry;
use std::path::Path;

#[derive(Default)]
pub struct ApodData {
    html_archive: Archive<ArchiveHtml>,
    entry_archive: Archive<ApodEntry>,
}

impl ApodData {
    pub fn load_html_archive(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.html_archive = Archive::load(path.as_ref())?;
        Ok(())
    }

    pub fn load_entry_archive(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.entry_archive = Archive::load(path.as_ref())?;
        Ok(())
    }
}
