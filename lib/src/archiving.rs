use crate::date::ApodDate;
use crate::INCLUDED_HTML_ARCHIVE;
use std::collections::HashMap;
use std::path::Path;
use zstd::zstd_safe::CompressionLevel;

pub mod html;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Codec error: {0}")]
    Codec(#[from] bitcode::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Archive<E: ArchiveEntry> {
    entries: HashMap<ApodDate, E>,
}

pub trait ArchiveEntry: bitcode::Encode + for<'a> bitcode::Decode<'a> + Clone {
    fn date(&self) -> ApodDate;
}

impl<E: ArchiveEntry> Default for Archive<E> {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl<E: ArchiveEntry> Archive<E> {
    pub fn new(entries: HashMap<ApodDate, E>) -> Self {
        Self { entries }
    }

    pub fn push(&mut self, entry: E) {
        self.entries.insert(entry.date(), entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn encode(&self) -> Vec<u8> {
        let entries: Vec<E> = self.entries.values().cloned().collect();
        bitcode::encode(&entries)
    }

    pub fn decode(data: &[u8]) -> Result<Self, ArchiveError> {
        let entries: Vec<E> = bitcode::decode(data)?;
        Ok(entries.into())
    }

    pub fn compress(&self, level: CompressionLevel) -> Vec<u8> {
        zstd::encode_all(self.encode().as_slice(), level).unwrap()
    }

    pub fn decompress(data: &[u8]) -> Result<Self, ArchiveError> {
        Self::decode(&zstd::decode_all(data)?)
    }

    pub fn save(
        &self,
        path: &Path,
        compression_level: CompressionLevel,
    ) -> Result<(), std::io::Error> {
        std::fs::write(path, self.compress(compression_level))
    }

    pub fn load(path: &Path) -> Result<Self, ArchiveError> {
        Self::decompress(&std::fs::read(path)?)
    }

    pub fn has_date(&self, date: ApodDate) -> bool {
        self.entries.contains_key(&date)
    }

    pub fn get(&self, date: ApodDate) -> Option<&E> {
        self.entries.get(&date)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ApodDate, &E)> {
        self.entries.iter()
    }

    pub fn latest_date(&self) -> Option<ApodDate> {
        self.entries.keys().copied().max()
    }

    #[cfg(feature = "include-html-archive")]
    pub fn load_included_html_archive() -> Self {
        Self::decompress(INCLUDED_HTML_ARCHIVE).expect("Failed to decode included archive")
    }
}

impl<E: ArchiveEntry> From<Vec<E>> for Archive<E> {
    fn from(value: Vec<E>) -> Self {
        let entries = value
            .into_iter()
            .map(|e| (e.date(), e))
            .collect::<HashMap<_, _>>();

        Self::new(entries)
    }
}
